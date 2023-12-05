/*
    body.rs
    ----------------------------------------
    Description:
    * Provides struct for all physical objects that are rendered in the simulation
    * Internally tracks properties (position, velocity, etc.)
 */
/* --------------------- IMPORTS -------------------- */
// Modules

// Crates
use crate::common::*;

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
pub struct Body {
    // Internal
    form:     BodyForm,
    position: Vector2,
    rotation: i64,
    origin:   Vector2,
    radius:   u32,
    // BodyForm::Polygon
    sides:    u32,
    vertices: Vec<Vertex>,
    edges:    Vec<Edge>,
    // Physics
    center:       Vector2, // Center of mass; measured from origin
    frozen:       bool,    //
    mass:         u32,
    velocity:     Vector2,
    ang_velocity: i32,
    air_friction: f64,
}


/* -------------------- FUNCTIONS ------------------- */

