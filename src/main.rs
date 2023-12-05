/*
    main.rs
    ----------------------------------------
    Description:
    * Serves as an abstraction layer on top of the App {}
    * Used to interact with public App {} functions (setting the scene, starting the loop, etc.)
 */
/* --------------------- IMPORTS -------------------- */
// Modules
mod app;
mod common;
// Crates
use app::App;
use common::*;

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */


/* -------------------- FUNCTIONS ------------------- */
fn main() {
    let my_app = App::new();
    let world = my_app.engine.world;

    println!("Hello, world! {}", my_app.name);
}
