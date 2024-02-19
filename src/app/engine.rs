/*
    engine.rs
    ----------------------------------------
    Description:
    * Handles the computational step of the simulation
    * Tracks all global parameters
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::app::collision::{CollisionDetector, CollisionResolver};
use crate::common::{TBodyRef, TSharedRef, Vector2, Vector2M};
use crate::v2;

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */

pub struct Engine {
    shared: TSharedRef,
    gravity: Vector2M<f64>,
    detector: CollisionDetector,
    resolver: CollisionResolver,
}

/* -------------------- FUNCTIONS ------------------- */
impl Engine {
    pub fn new(shared: TSharedRef) -> Self {
        Engine {
            shared: shared.clone(),
            gravity: v2!(0f64, 1f64, 0.001),
            detector: CollisionDetector::new(shared.clone()),
            resolver: CollisionResolver::new(shared.clone()),

        }
    }

    pub fn step(&mut self, bodies: &Vec<TBodyRef>, delta: u64) {
        // Resolve gravity
        for body_ref in bodies {
            self.resolve_gravity(body_ref);
        }

        // Update body position/rotation
        for body_ref in bodies {
            let mut body = body_ref.borrow_mut();
            body.update(delta);
        }

        // TODO: Resolve constraints

        // TODO: Resolve collisions
        let result = self.detector.evaluate(bodies);
        self.resolver.resolve(result);
    }

    fn resolve_gravity(&self, body: &TBodyRef) {
        let mut body = body.borrow_mut();

        if body.frozen {
            return;
        }

        let gravity = self.gravity.to_vec2() * Vector2::from(self.gravity.m);
        body.velocity = body.velocity + gravity * body.mass;
    }
}

