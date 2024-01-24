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
use crate::app::objects::Body;
use crate::common::{AABB, Vector2};
use std::cell::RefCell;
use std::rc::Rc;
use crate::v2;


/* -------------------- VARIABLES ------------------- */
const THRESHOLD: usize = 2;
const GRID_SIZE: (usize, usize) = (20, 20);
type BodyContainer = Option<Vec<Rc<RefCell<Body>>>>; // Contains references to bodies within a given cell

/* ------------------- STRUCTURES ------------------- */


pub struct CollisionDetector {
    cells: Vec<BodyContainer>,
}


/* -------------------- FUNCTIONS ------------------- */
impl CollisionDetector {
    pub fn new() -> Self {
        CollisionDetector {
            cells: vec![None; GRID_SIZE.0 * GRID_SIZE.1],
        }
    }

    pub fn evaluate(&self, bodies: &Vec<Body>, window_size: (u32, u32)) {
        // let result = self.broad_phase(bodies, window_size);
        // // dbg!(result);
        //
        // self.narrow_phase();
    }

    fn broad_phase(&self, bodies: &Vec<Body>, window_size: (u32, u32)) -> Vec<Body> {
        todo!()
    }

    fn narrow_phase(&self) {}
}

