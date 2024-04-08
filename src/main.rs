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
use crate::common::{BodyForm, Vector2, Materials};
use crate::app::{App, objects::Body};

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */


/* -------------------- FUNCTIONS ------------------- */
fn main() {
    let mut my_app = App::new("DEFAULT", 1000, 600);

    // PENTAGON SETUP
    let bodies = vec![
        rect!(v2!(50, 250), 500, 50, Materials::BOUNCY).set_frozen(true).set_rotation(PI / 12f64),
        poly!(v2!(50, 0), 50, 5),
        // poly!(v2!(250, 250), 80, 4).set_rotation(PI / 4f64),
        // poly!(v2!(250, 0), 80, 4).set_rotation(PI / 4f64),
        // poly!(v2!(250, 100), 80, 4).set_rotation(PI / 4f64),
        // poly!(v2!(250, 350), 80, 4).set_rotation(PI / 4f64),
        // poly!(v2!(450, 0), 50, 20, Materials::BOUNCY),
        // poly!(v2!(500, 0), 120, 3),
        // poly!(v2!(640, 250), 30, 3).set_rotation(PI / 3.0),
    ];


    // RECT SETUP
    // let bodies = vec![
    //     rect!(v2!(370, 300), 200, 100).set_frozen(true),
    //     rect!(v2!(450, 170), 100, 50, 1.5),
    // ];

    my_app.add_bodies(bodies);

    my_app.start();
}
