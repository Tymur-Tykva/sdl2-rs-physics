/*
    detector.rs
    ----------------------------------------
    Description:
    * Provides methods to resolve collision
    * Broad phase uses BSP (Binary Search Partitioning)
    * Narrow phase uses SAT (Separating Axis Theorem)
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::common::{ConvertPrimitives, Crd, Vector2, GRID_SIZE, MutShared};
use crate::app::objects::Body;
use std::rc::Rc;

/* -------------------- VARIABLES ------------------- */
const THRESHOLD: usize = 2;
// const GRID_SIZE: Vector2<usize> = v2!(5, 5);

/* ------------------- STRUCTURES ------------------- */


pub struct CollisionDetector {
    shared: MutShared,
    cells: Vec<Vec<Option<Rc<Body>>>>,    // 2D vector of AABBs
    out_of_bounds: Vec<Option<Rc<Body>>>, // 1D vector of AABBs which are out of bounds,
                                              // but still should be accounted for in collision
}


/* -------------------- FUNCTIONS ------------------- */
impl CollisionDetector {
    pub fn new(shared: MutShared) -> Self {
        CollisionDetector {
            shared,
            cells: vec![vec![None; GRID_SIZE.y]; GRID_SIZE.x],
            out_of_bounds: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, bodies: &Vec<Body>) {
        self.cells = vec![vec![None; GRID_SIZE.y]; GRID_SIZE.x];
        self.out_of_bounds = Vec::new();

        self.broad_phase(bodies);
    }

    fn broad_phase(&mut self, bodies: &Vec<Body>) -> Vec<Body> {
        let window_size = self.shared.borrow_mut().window_size;
        let bounds: Vector2<Crd> = (window_size / GRID_SIZE.to()).to();

        for body in bodies {
            // Evaluate whether body out of bounds
            let mut grid = vec![];
            let aabb = body.aabb();

            for point in aabb.clone().points {
                let point: Vector2<Crd> = (point / bounds).to();
                grid.push(point);
            }

            let max_x = grid.iter().map(|v2| v2.x).max().unwrap_or(-1);
            let max_y = grid.iter().map(|v2| v2.y).max().unwrap_or(-1);
            let min_x = grid.iter().map(|v2| v2.x).min().unwrap_or(-1);
            let min_y = grid.iter().map(|v2| v2.y).min().unwrap_or(-1);

            for i in min_x..=max_x {
                for j in min_y..=max_y {
                    let i = i as usize;
                    let j = j as usize;

                    self.cells[j][i] = Some(Rc::from(body));
                }
            }

            println!("{}, {} | {}, {}", max_x, max_y, min_x, min_y)
        }

        vec![]
    }

    fn narrow_phase(&self) {}
}

