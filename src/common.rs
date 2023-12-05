/*
    common.rs
    ----------------------------------------
    Description:
    * Provides access to commonly-used functions, data types, etc. to the physics engine
 */
/* --------------------- IMPORTS -------------------- */


/* -------------------- VARIABLES ------------------- */
pub type Tcrd = i16;


/* ------------------- STRUCTURES ------------------- */
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: Tcrd,
    pub y: Tcrd,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2M {
    pub x: Tcrd,
    pub y: Tcrd,
    pub m: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub id: i64,
    pub x: Tcrd,
    pub y: Tcrd,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub points: Vec<Vertex>,
    pub internal: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum BodyForm {
    Polygon,
    Circle
}


/* -------------------- FUNCTIONS ------------------- */

