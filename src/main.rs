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

use std::f64::consts::PI;
// Crates
use crate::common::{BodyForm, Vector2};
use crate::app::{App, objects::Body};

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */


/* -------------------- FUNCTIONS ------------------- */
fn main() {
    let mut my_app = App::new("DEFAULT", 1000, 600);

    // PENTAGON SETUP
    let bodies = vec![
        rect!(v2!(50, 250), 500, 50).set_frozen(true).set_rotation(PI / 12f64),
        poly!(v2!(50, 0), 50, 5),
        // poly!(v2!(250, 250), 80, 4, 1.0),
        // poly!(v2!(450, 250), 50, 7, 1.0),
        // poly!(v2!(500, 250), 120, 3, 1.0),
        // poly!(v2!(640, 250), 30, 3, 1.0).set_rotation(PI / 3.0),
    ];


    // RECT SETUP
    // let bodies = vec![
    //     rect!(v2!(370, 300), 200, 100).set_frozen(true),
    //     rect!(v2!(450, 170), 100, 50, 1.5),
    // ];

    my_app.add_bodies(bodies);

    my_app.start();
}
