/*
    engine.rs
    ----------------------------------------
    Description:
    * Handles the computational step of the simulation
    * Tracks all global parameters
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::app::objects::Collection;
use crate::common::*;


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
pub struct Engine<'a> {
    pub world: Collection<'a>,
    pub gravity: Vector2M,
}


/* -------------------- FUNCTIONS ------------------- */
impl Engine {
    pub fn new() -> Self {
        Engine {
            world: Collection::new(Some("World")),
            gravity: Vector2M { x: 0, y: 1, m: 0.01 }
        }
    }
}

