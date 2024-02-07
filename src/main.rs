/*
    main.rs
    ----------------------------------------
    Description:
    * Serves as an abstraction layer on top of the App {}
    * Used to interact with public App {} functions (setting the scene, starting the loop, etc.)
 */
/* --------------------- IMPORTS -------------------- */
// Modules
pub mod app;
pub mod common;
// Crates
use crate::app::{App, objects::Body};
use crate::common::{BodyForm, Vector2};

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */


/* -------------------- FUNCTIONS ------------------- */
fn main() {
    let mut my_app = App::new("DEFAULT", 1000, 600);

    let bodies = vec![
        poly!(v2!(400, 200), 100, 5),
        poly!(v2!(500, 200), 50, 5),
    ];
    my_app.add_bodies(bodies);

    my_app.start();
}
