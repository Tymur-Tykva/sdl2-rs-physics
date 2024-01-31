/*
    engine.rs
    ----------------------------------------
    Description:
    * Handles the computational step of the simulation
    * Tracks all global parameters
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::app::collision::CollisionDetector;
use crate::common::{TBodyRef, TSharedRef, Vector2, Vector2M};
use crate::v2;

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */

pub struct Engine {
    shared: TSharedRef,
    gravity: Vector2M<f64>,
    collision: CollisionDetector,
}

/* -------------------- FUNCTIONS ------------------- */
impl Engine {
    pub fn new(shared: TSharedRef) -> Self {
        Engine {
            shared: shared.clone(),
            gravity: v2!(0f64, 1f64, 0.001),
            collision: CollisionDetector::new(shared.clone()),
        }
    }

    pub fn step(&mut self, bodies: &Vec<TBodyRef>, delta: u64) {
        // Resolve gravity
        for body_ref in bodies {
            // self.resolve_gravity(body_ref);
        }

        // Update body position/rotation
        for body_ref in bodies {
            let mut body = body_ref.borrow_mut();
            body.update(delta);
        }

        // TODO: Resolve constraints

        // TODO: Resolve collisions
        self.collision.evaluate(bodies);
    }

    fn resolve_gravity(&self, body: &TBodyRef) {
        let mut body = body.borrow_mut();

        if body.frozen() {
            return;
        }

        let gravity = self.gravity.to_vec2() * Vector2::from(self.gravity.m);
        let force = gravity + body.force_buffer(); // TODO: factor-in body mass (*body.mass)
        body.set_force_buffer(force);
    }
}

