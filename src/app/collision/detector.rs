/*
    detector.rs
    ----------------------------------------
    Description:
    * Provides methods to resolve collision
    * Broad phase uses a scaled grid
    * Narrow phase uses SAT (Separating Axis Theorem)
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::app::objects::Body;
use crate::common::{ConvertPrimitives, Disp, GRID_SIZE, TBodyRef, TCollisionPairs, TCollisionGrid, TSharedRef, Vector2, Crd, CollisionResult, Projection, Axis, Vertex, almost_eq};
use crate::v2;

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
pub struct CollisionDetector {
    shared: TSharedRef,
    collision_grid: TCollisionGrid, // 2D vector of AABBs
    out_of_bounds: Vec<TBodyRef>,   // 1D vector of AABBs which are out of bounds, but still should be accounted for in collision
}

/* -------------------- FUNCTIONS ------------------- */
impl CollisionDetector {
    pub fn new(shared: TSharedRef) -> Self {
        CollisionDetector {
            shared,
            collision_grid: vec![vec![vec![]; GRID_SIZE.y]; GRID_SIZE.x],
            out_of_bounds: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, bodies: &Vec<TBodyRef>) -> Vec<CollisionResult> {
        self.collision_grid = vec![vec![vec![]; GRID_SIZE.y]; GRID_SIZE.x];
        self.out_of_bounds = Vec::new();

        let candidate_pairs = self.broad_phase(bodies);
        let colliding_pairs = self.narrow_phase(candidate_pairs);

        // println!("Colliding={:?}", colliding_pairs);
        colliding_pairs
    }

    /// Returns object pairs for more precise analysis in the narrow phase
    fn broad_phase(&mut self, bodies: &Vec<TBodyRef>) -> TCollisionPairs {
        // TODO: Destroy objects too far out of bounds

        let window_size: Vector2<Crd> = self.shared.borrow_mut().window_size.to();
        let bounds: Vector2<Crd> = window_size / GRID_SIZE.to();
        // Broad-phase results
        let mut marked: Vec<(usize, usize)> = Vec::new();
        let mut pairs: TCollisionPairs = Vec::new();

        for body_ref in bodies {
            let body = body_ref.borrow_mut();

            // Evaluate whether body out of bounds
            let mut points = vec![];
            let aabb = body.aabb();

            for point in aabb.clone().points {
                let point: Vector2<Crd> = (point / bounds).to();
                points.push(point);
            }

            // Find maximum and minimum points
            let max_x= points.iter().map(|p| p.disp().x).max().unwrap_or(-1);
            let max_y= points.iter().map(|p| p.disp().y).max().unwrap_or(-1);
            let min_x= points.iter().map(|p| p.disp().x).min().unwrap_or(-1);
            let min_y= points.iter().map(|p| p.disp().y).min().unwrap_or(-1);

            // Fill grid
            let grid_size_crd: Vector2<Disp> = GRID_SIZE.to();
            let mut oob = false;

            for i in min_y..=max_y { for j in min_x..=max_x {
                // Allow not completely OOB objects to interact with collision
                if i >= 0 && i < grid_size_crd.y && j >= 0 && j < grid_size_crd.x {
                    let i = i as usize;
                    let j = j as usize;
                    self.collision_grid[i][j].push(body_ref.clone());

                    // If cell has >= 2 objects, mark it as a collision candidate
                    if self.collision_grid[i][j].len() < 2 || marked.contains(&(i, j)) { continue; }
                    marked.push((i, j));
                } else {
                    if !oob {
                        oob = true;
                        self.out_of_bounds.push(body_ref.clone());
                    }
                }
            }}
        }

        // Update shared collision grid information
        self.shared.borrow_mut().collision_grid = self.collision_grid.clone();

        // Fetch in-bound collision pairs
        for n in 0..marked.len() {
            let (i, j) = marked[n];
            let cell = self.collision_grid[i][j].clone();

            // Iterate through all possible cell permutations
            for a in 0..cell.len() { for b in 1..cell.len() {
                // Ensure no duplicates
                if cell[a] == cell[b]
                    || pairs.contains(&[cell[a].clone(), cell[b].clone()])
                    || pairs.contains(&[cell[b].clone(), cell[a].clone()])
                    || cell[a].borrow().ignore_groups.contains(&cell[b].borrow().collision_group)
                    || cell[b].borrow().ignore_groups.contains(&cell[a].borrow().collision_group)
                { continue; }

                pairs.push([cell[a].clone(), cell[b].clone()]);
            }}
        }

        // Fetch out-of-bounds collision pairs
        for a in 0..self.out_of_bounds.len() { for b in 1..self.out_of_bounds.len() {
            if self.out_of_bounds[a] == self.out_of_bounds[b]
                || pairs.contains(&[self.out_of_bounds[a].clone(), self.out_of_bounds[b].clone()])
                || pairs.contains(&[self.out_of_bounds[b].clone(), self.out_of_bounds[a].clone()])
                || self.out_of_bounds[a].borrow().ignore_groups.contains(&self.out_of_bounds[b].borrow().collision_group)
                || self.out_of_bounds[b].borrow().ignore_groups.contains(&self.out_of_bounds[a].borrow().collision_group)
            { continue; }

            pairs.push([self.out_of_bounds[a].clone(), self.out_of_bounds[b].clone()]);
        }}

        // Update shared broad-phase pair information
        self.shared.borrow_mut().broad_phase_pairs = pairs.clone();

        pairs
    }

    /// Confirm/deny collision using the Separating Axis Theorem (SAT)
    fn narrow_phase(&self, pairs: TCollisionPairs) -> Vec<CollisionResult> {
        let mut colliding_pairs: Vec<CollisionResult> = Vec::new();

        for pair in pairs {
            let body1 = pair[0].borrow();
            let body2 = pair[1].borrow();
            // Collision result
            let mut colliding = true;
            let mut min_overlap: f64 = -1.0;
            let mut min_axis: Vector2<f64> = v2!(-1.0);
            // let mut min_point: Vector2<f64> = v2!(0.0);

            // Get all non-duplicate axes
            let mut axes: Vec<Axis> = body1.axes().iter().map(|&ax| Axis { v2: ax, parent: pair[0].clone() }).collect();
            for axis in body2.axes() {
                let ax = Axis { v2: axis, parent: pair[1].clone() };
                if axes.contains(&ax) { continue; }
                axes.push(ax);
            }

            // Check whether points overlap in axis projection
            for axis in axes {
                let ax = axis.v2;
                let proj_1 = self.projection_bounds(&body1, ax.norm());
                let proj_2 = self.projection_bounds(&body2, ax.norm());

                // Check if they are colliding
                if proj_1.max <= proj_2.min || proj_2.max <= proj_1.min {
                    colliding = false;
                    break;
                } else {
                    // Update minimum overlap
                    let overlap = proj_1.max - proj_2.min;

                    if min_overlap == -1.0 || overlap < min_overlap {
                        min_overlap = overlap;
                        min_axis = ax.norm();
                    }
                }
            }

            if colliding {
                let contacts = self.find_contacts(&body1, &body2);

                let proj_1 = self.projection_bounds(&body1, min_axis);
                let proj_2 = self.projection_bounds(&body2, min_axis);

                let colliding_pair = CollisionResult {
                    bodies: pair.clone(),
                    normal: min_axis,
                    overlap: min_overlap,
                    contacts,
                };

                colliding_pairs.push(colliding_pair);
            }
        }

        // Update shared narrow-phase collision pair indicator
        self.shared.borrow_mut().narrow_phase_pairs = colliding_pairs.clone();

        colliding_pairs
    }

    /// Find the min/max points of body projected onto a given axis
    fn projection_bounds(&self, body: &Body, axis: Vector2<f64>) -> Projection {
        let vertices: Vec<Vector2<f64>> = body.vertices.clone().iter().map(|vtx| body.globalise(vtx.to_vec2()).to()).collect();

        // let proj = Vector2::dot(vertices[0], axis);
        // let mut max: f64 = proj;
        // let mut min: f64 = proj;
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        // Get bounds over polygon`
        for i in 0..body.sides as usize {
            let proj = Vector2::dot(vertices[i], axis);
            if proj < min {
                min = proj;
            }
            if proj > max {
                max = proj;
            }
        }

        // println!("{min}, {max} | {:?}, {:?}", p_min, p_max);
        Projection { min, max }
    }

    /// Find collision contact points
    fn find_contacts(&self, b1: &Body, b2: &Body) -> Vec<Vector2<f64>> {
        let v1: Vec<Vector2<f64>> = b1.vertices.clone().iter().map(|v| b1.globalise(v.to_vec2()).to()).collect();
        let v2: Vec<Vector2<f64>> = b2.vertices.clone().iter().map(|v| b2.globalise(v.to_vec2()).to()).collect();

        let mut contacts: Vec<Vector2<f64>> = vec![v2!(0.0), v2!(0.0)];
        let mut min_dist: f64 = 10f64.powi(10);

        for i in 0..v1.len() {
            let p = v1[i];

            for j in 0..v2.len() {
                let l1 = v2[j];
                let l2 = v2[if j + 1 == v2.len() { 0 } else { j + 1 }];

                let (d, contact) = Vector2::<f64>::p_dist(p, l1, l2);

                // println!("d={}, md={}, eq={}", d.sqrt(), min_dist.sqrt(), almost_eq(d.sqrt(), min_dist.sqrt()));

                if almost_eq(d, min_dist) {
                    if !Vector2::<f64>::almost_eq(contact, contacts[0]) {
                       contacts[1] = contact;
                    }
                } else if d < min_dist {
                    min_dist = d;
                    contacts[0] = contact;
                    contacts[1] = v2!(0f64);
                }
            }
        }

        for i in 0..v2.len() {
            let p = v2[i];

            for j in 0..v1.len() {
                let l1 = v1[j];
                let l2 = v1[if j + 1 == v1.len() { 0 } else { j + 1 }];

                let (d, contact) = Vector2::<f64>::p_dist(p, l1, l2);

                // println!("d={}, md={}, eq={}", d.sqrt(), min_dist.sqrt(), almost_eq(d.sqrt(), min_dist.sqrt()));

                if almost_eq(d, min_dist) {
                    if !Vector2::<f64>::almost_eq(contact, contacts[0]) {
                        contacts[1] = contact;
                    }
                } else if d < min_dist {
                    min_dist = d;
                    contacts[0] = contact;
                    contacts[1] = v2!(0f64);
                }
            }
        }

        // Remove all occurrences of (0, 0)
        for i in 0..contacts.len() {
            let el = contacts[i];
            if el.x == 0f64 && el.y == 0f64 {
                contacts.remove(i);
            }
        }

        contacts
    }
}

