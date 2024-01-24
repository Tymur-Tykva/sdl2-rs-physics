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
use crate::app::objects::CAddInput::*;
use crate::common::{Vector2, BodyForm};


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */


/* -------------------- FUNCTIONS ------------------- */
fn main() {
    let mut my_app = App::new("DEFAULT", 1000, 600);

    let bodies = Bodies(vec![
        rect!(v2!(100, 105), 100, 100),
        rect!(v2!(250, 293), 100, 100),
        rect!(v2!(400, 290), 100, 100),
        rect!(v2!(550, 315), 100, 100),
        rect!(v2!(700, 164), 100, 100),
        rect!(v2!(850, 339), 100, 100),
    ]);
    my_app.world.add(bodies);

    my_app.start();
}
