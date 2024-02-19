/*
    resolver.rs
    ----------------------------------------
    Description:
    * Provides method to resolve collisions detected by SAT
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::common::{CollisionResult, TSharedRef, Vector2};
use crate::v2;


/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
pub struct CollisionResolver {
    shared: TSharedRef,
}

/* -------------------- FUNCTIONS ------------------- */
const CORRECTION_PERCENTAGE: f64 = 0.2;
const SLOP: f64 = 0.01;

impl CollisionResolver {
    pub fn new(shared: TSharedRef) -> Self {
        CollisionResolver {
            shared
        }
    }

    pub fn resolve(&self, collisions: Vec<CollisionResult>) {
        for result in collisions {
            let mut b1 = result.bodies[0].borrow_mut();
            let mut b2 = result.bodies[1].borrow_mut();
            let n = result.normal.norm();
            let overlap = result.overlap;

            // Evaluate the bodies' positions
            if Vector2::dot(b1.globalise(v2!(0.0)), n) >= Vector2::dot(b2.globalise(v2!(0.0)), n) {
                (b1, b2) = (b2, b1);
            }

            // Pre-resolution setup
            let rel_v = b2.velocity - b1.velocity;
            let v_norm = Vector2::dot(rel_v, n);

            let e = if b1.restitution <= b2.restitution { b1.restitution } else { b2.restitution };
            let b1_inv_mass = 1.0 / b1.mass;
            let b2_inv_mass = 1.0 / b2.mass;

            // Calculate impulse magnitude required to resolve collision
            let m = (v_norm * -(e + 1.0))/(b1_inv_mass + b2_inv_mass);
            let mut impulse = n * m;
            // Calculate positional correction
            let corrected_overlap = if overlap - SLOP > 0.0 { overlap - SLOP } else { 0.0 };
            let mut correction = v2!(corrected_overlap) / (n * (b1_inv_mass + b2_inv_mass) * CORRECTION_PERCENTAGE);

            // Account for a frozen body not separating
            if b1.frozen || b2.frozen {
                impulse = impulse * 2.0;
                correction = correction * 2.0;
            }

            // Apply impulse resolution & positional correction
            if !b1.frozen {
                b1.velocity = b1.velocity - (impulse * b1_inv_mass);
                b1.position = b1.position - correction * b1_inv_mass;
            }
            if !b2.frozen {
                b2.velocity = b2.velocity + (impulse * b2_inv_mass);
                b2.position = b2.position + correction * b2_inv_mass;
            }

            // Friction
            let rel_v = b2.velocity - b1.velocity; // Update relative velocity
            let t = rel_v - n * Vector2::dot(rel_v, n); // Calculate tangent
            let v_tan = Vector2::dot(rel_v, t);
            let m = (v_tan * -(e + 1.0))/(b1_inv_mass + b2_inv_mass);

            let jt = -Vector2::dot(rel_v, t) / (b1_inv_mass + b2_inv_mass);
            let k_static = (b1.initial_friction.powi(2) + b2.initial_friction.powi(2)).sqrt();
            let k_continuous = (b1.continuous_friction.powi(2) + b2.continuous_friction.powi(2)).sqrt();

            let mut friction_impulse;
            println!("m={:?}\nk_static={:?}", m, k_static);
            if jt.abs() < m * k_static {
                println!("Static");
                friction_impulse = t * jt;
            } else {
                println!("Dynamic");
                friction_impulse = t * -m * k_continuous;
            }
            println!("f_imp={:?}", friction_impulse);

            if b1.frozen || b2.frozen {
                friction_impulse = friction_impulse * 2.0;
            }

            if !b1.frozen {
                b1.velocity = b1.velocity + (friction_impulse * b1_inv_mass);
            }
            if !b2.frozen {
                b2.velocity = b2.velocity + (friction_impulse * b2_inv_mass);
            }
        }
    }
}
