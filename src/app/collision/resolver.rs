/*
    resolver.rs
    ----------------------------------------
    Description:
    * Provides method to resolve collisions detected by SAT
 */
/* --------------------- IMPORTS -------------------- */
// Modules

// Crates
use crate::common::{TCollisionPairs, TSharedRef};
use std::cmp::min;


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
struct CollisionResolver {
    shared: TSharedRef,

}

/* -------------------- FUNCTIONS ------------------- */
impl CollisionResolver {
    pub fn new(shared: TSharedRef) -> Self {
        CollisionResolver {
            shared
        }
    }

    pub fn resolve(collisions: TCollisionPairs) {
        for pair in collisions {
            let b1 = pair[0].borrow_mut();
            let b2 = pair[1].borrow_mut();

            let e = if b1.restitution() <= b2.restitution() { b1.restitution() } else { b2.restitution() };
            // let m = -(1 + e)(b2.position() - b1.position()); // *n)/(1/b1.mass + 1/b2.mass)
        }
    }
}

