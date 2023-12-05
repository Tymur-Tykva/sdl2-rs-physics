/*
    app/mod.rs
    ----------------------------------------
    Description:
    * Central file for the physics engine
    * Provides the App struct itself, and manages all internal components of the physics engine
      (rendering, computation, etc.)
    * Manages the 'main loop'
    * TODO: consider function 'injection' into the main loop; inline candidate
 */
/* --------------------- IMPORTS -------------------- */
// Modules
pub mod objects;
pub mod render;
mod engine;
// Crates
pub use engine::Engine;


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
pub struct App<'a> {
    pub name: &'a str,
    pub engine: Engine<'a>,
}


/* -------------------- FUNCTIONS ------------------- */
impl App<'_> {
    pub fn new() -> Self {
        App {
            name: "test",
            engine: Engine::new(),
        }
    }
}

