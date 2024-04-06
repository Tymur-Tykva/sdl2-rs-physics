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
const ITERATIONS: u32 = 1;


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
            gravity: v2!(0f64, 1f64, 9.81),
            detector: CollisionDetector::new(shared.clone()),
            resolver: CollisionResolver::new(shared.clone()),

        }
    }

    pub fn step(&mut self, bodies: &Vec<TBodyRef>, dt: f64) {
        let dt = dt / (ITERATIONS as f64);

        // for _ in 0..ITERATIONS {
            // Resolve gravity
            for body_ref in bodies {
                if body_ref.borrow().frozen { continue; }
                self.resolve_gravity(body_ref, dt);
            }

            // Update body position/rotation
            for body_ref in bodies {
                let mut body = body_ref.borrow_mut();
                body.update(dt);
            }

            // TODO: Resolve constraints

            let result = self.detector.evaluate(bodies);
            self.resolver.resolve(result);
        // }
    }

    fn resolve_gravity(&self, body: &TBodyRef, dt: f64) {
        let mut body = body.borrow_mut();

        if body.frozen {
            return;
        }

        let gravity = self.gravity.to_vec2() * self.gravity.m;
        body.velocity = body.velocity + gravity;
    }
}

