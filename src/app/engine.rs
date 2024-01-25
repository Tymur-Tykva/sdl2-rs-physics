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
use crate::app::objects::{Body, Collection};
use crate::common::{MutShared, Vector2, Vector2M};
use crate::v2;


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */

pub struct Engine<'a> {
    shared: MutShared,
    gravity: Vector2M<f64>,
    collision: CollisionDetector<'a>,
}


/* -------------------- FUNCTIONS ------------------- */
impl Engine<'_> {
    pub fn new(shared: MutShared) -> Self {
        Engine {
            shared: shared.clone(),
            gravity: v2!(0f64, 1f64, 0.001),
            collision: CollisionDetector::new(shared.clone()),
        }
    }

    pub fn step(&mut self, world: &mut Collection, delta: u64) {
        let bodies = world.bodies();

        // Resolve gravity
        for body in world.mut_bodies() {
            // self.resolve_gravity(body);
        }

        // Update body position/rotation
        for body in world.mut_bodies() {
            body.update(delta);
        }

        // TODO: Resolve constraints

        // TODO: Resolve collisions
        self.collision.evaluate(world.bodies());
    }

    fn resolve_gravity(&self, body: &mut Body) {
        if body.frozen() {
            return;
        }

        let gravity = self.gravity.to_vec2() * Vector2::from(self.gravity.m);
        let force = gravity + body.force_buffer(); // TODO: factor-in body mass (*body.mass)
        body.set_force_buffer(force);
    }
}

