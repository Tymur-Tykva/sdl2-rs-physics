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
        poly!(v2!(431, 203), 117, 6),
        poly!(v2!(680, 78), 83, 5),
        // poly!(v2!(750, 50), 40, 7),
        // rect!(v2!(700, 164), 100, 100),
        // rect!(v2!(100, 105), 100, 100),
        // rect!(v2!(250, 293), 100, 100),
        // rect!(v2!(400, 290), 100, 100),
    ];
    my_app.add_bodies(bodies);

    my_app.start();
}
