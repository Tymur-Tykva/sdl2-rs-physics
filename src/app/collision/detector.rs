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
use crate::common::{ConvertPrimitives, Crd, GRID_SIZE, TBodyRef, TCollisionCandidatePairs, TCollisionGrid, TSharedRef, Vector2};

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

        self.broad_phase(bodies);
    }

    /// Returns object pairs for more precise analysis in the narrow phase
    fn broad_phase(&mut self, bodies: &Vec<TBodyRef>) -> TCollisionCandidatePairs {
        let window_size = self.shared.borrow_mut().window_size;
        let bounds: Vector2<Crd> = (window_size / GRID_SIZE.to()).to();
        // Broad-phase results
        let mut marked: Vec<(usize, usize)> = Vec::new();
        let mut candidates: TCollisionCandidatePairs = Vec::new();


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

            // Ensure object within display bounds
            let grid_size_crd: Vector2<Crd> = GRID_SIZE.to();
            if min_x >= 0 && min_y >= 0 && max_x < grid_size_crd.x && max_y < grid_size_crd.y {
                // Fill grid with object reference
                for i in min_y..=max_y { for j in min_x..=max_x {
                    let i = i as usize;
                    let j = j as usize;
                    self.collision_grid[i][j].push(body_ref.clone());

                    // If cell has >= 2 objects, mark it as a collision candidate
                    if self.collision_grid[i][j].len() < 2 || marked.contains(&(i, j)) { continue; }
                    marked.push((i, j));
                }}
            } else {
                self.out_of_bounds.push(body_ref.clone())
            }

        }

        // Update shared collision grid information
        self.shared.borrow_mut().collision_grid = self.collision_grid.clone();

        // Fetch collision pairs
        for n in 0..marked.len() {
            let (i, j) = marked[n];
            let cell = self.collision_grid[i][j].clone();

            // Iterate through all possible cell permutations
            for a in 0..cell.len() { for b in 1..cell.len() {
                // Ensure no duplicates
                if cell[a] == cell[b]
                    || candidates.contains(&[cell[a].clone(), cell[b].clone()])
                    || candidates.contains(&[cell[b].clone(), cell[a].clone()]) { continue; }

                println!("{i} {j} | {a} {b} : {:?}", [cell[a].clone(), cell[b].clone()].clone().map(|b_ref| b_ref.borrow().sides()));
                candidates.push([cell[a].clone(), cell[b].clone()]);
            }}
        }

        println!("=========");
        candidates
    }

    fn narrow_phase(&self) {}
}

