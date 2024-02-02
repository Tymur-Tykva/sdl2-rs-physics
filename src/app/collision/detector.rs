/*
    detector.rs
    ----------------------------------------
    Description:
    * Provides methods to resolve collision
    * Broad phase uses a scaled grid
    * Narrow phase uses SAT (Separating Axis Theorem)
 */
/* --------------------- IMPORTS -------------------- */
use crate::app::objects::Body;
// Crates
use crate::common::{ConvertPrimitives, Crd, GRID_SIZE, TBodyRef, TCollisionPairs, TCollisionGrid, TSharedRef, Vector2};

/* -------------------- VARIABLES ------------------- */
const THRESHOLD: usize = 2;

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

    pub fn evaluate(&mut self, bodies: &Vec<TBodyRef>) {
        self.collision_grid = vec![vec![vec![]; GRID_SIZE.y]; GRID_SIZE.x];
        self.out_of_bounds = Vec::new();

        let candidate_pairs = self.broad_phase(bodies);
        let colliding_pairs = self.narrow_phase(candidate_pairs);

        // println!("{:?}", colliding_pairs);
    }

    /// Returns object pairs for more precise analysis in the narrow phase
    fn broad_phase(&mut self, bodies: &Vec<TBodyRef>) -> TCollisionPairs {
        // TODO: Destroy objects too far out of bounds

        let window_size = self.shared.borrow_mut().window_size;
        let bounds: Vector2<Crd> = (window_size / GRID_SIZE.to()).to();
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
            let max_x = points.iter().map(|v2| v2.x).max().unwrap_or(-1);
            let max_y = points.iter().map(|v2| v2.y).max().unwrap_or(-1);
            let min_x = points.iter().map(|v2| v2.x).min().unwrap_or(-1);
            let min_y = points.iter().map(|v2| v2.y).min().unwrap_or(-1);

            // Fill grid
            let grid_size_crd: Vector2<Crd> = GRID_SIZE.to();
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
                    || pairs.contains(&[cell[b].clone(), cell[a].clone()]) { continue; }

                pairs.push([cell[a].clone(), cell[b].clone()]);
            }}
        }

        // Fetch out-of-bounds collision pairs
        for a in 0..self.out_of_bounds.len() { for b in 1..self.out_of_bounds.len() {
            if self.out_of_bounds[a] == self.out_of_bounds[b]
                || pairs.contains(&[self.out_of_bounds[a].clone(), self.out_of_bounds[b].clone()])
                || pairs.contains(&[self.out_of_bounds[b].clone(), self.out_of_bounds[a].clone()]) { continue; }

            pairs.push([self.out_of_bounds[a].clone(), self.out_of_bounds[b].clone()]);
        }}

        // Update shared broad-phase pair information
        self.shared.borrow_mut().broad_phase_pairs = pairs.clone();

        pairs
    }

    /// Confirm/deny collision using the Separating Axis Theorem (SAT)
    fn narrow_phase(&self, pairs: TCollisionPairs) -> TCollisionPairs {
        let colliding_pairs: TCollisionPairs = Vec::new();

        for pair in pairs {
            let body1 = pair[0].borrow();
            let body2 = pair[1].borrow();
            let mut colliding = true;

            // Get all non-duplicate axes
            let mut axes = body1.axes();
            for axis in body2.axes() {
                if axes.contains(&axis) { continue; }
                axes.push(axis)
            }

            for axis in axes {
                let b1_bounds = self.projection_bounds(&body1, axis);
                let b2_bounds = self.projection_bounds(&body2, axis);

                if b1_bounds.1 < b2_bounds.0 || b2_bounds.1 < b1_bounds.0 {
                    colliding = false;
                    break;
                }
            }

            if colliding {
                println!("{} and {} are colliding", body1.sides(), body2.sides());

            } else {
                println!("{} and {} are not colliding", body1.sides(), body2.sides());
            }
        }

        colliding_pairs
    }

    fn projection_bounds(&self, body: &Body, axis: Vector2<Crd>) -> (Crd, Crd) {
        let proj = Vector2::dot(axis, body.vertices()[0].to_vec2());
        let mut max: Crd = 0;
        let mut min: Crd = proj;

        // Get bounds over polygon`
        for i in 1..body.sides() as usize {
            let proj = Vector2::dot(axis, body.vertices()[i].to_vec2());

            if proj < min {
                min = proj;
            } else if proj > max {
                max = proj;
            }
        }

        (min, max)
    }
}

