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
const CORRECTION_PERCENTAGE: f64 = 0.6;
const SLOP: f64 = 0.02;

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
            let n = result.normal;
            let overlap = result.overlap;

            // Evaluate the bodies' positions
            if Vector2::dot(b1.globalise(v2!(0.0)), n) >= Vector2::dot(b2.globalise(v2!(0.0)), n) {
                (b1, b2) = (b2, b1);
            }

            println!("== Resolution ==");
            println!("b1");
            b1.ident();
            println!("b2");
            b2.ident();
            println!("====");
            println!("normal={:?}\noverlap={overlap}\n", n);

            // Pre-resolution setup
            let rel_v = b2.velocity - b1.velocity;
            let v_norm = Vector2::dot(rel_v, n);

            println!("rv={:?}\nv_norm={v_norm}", rel_v);

            let e = if b1.restitution <= b2.restitution { b1.restitution } else { b2.restitution };
            let b1_inv_mass = 1.0 / b1.mass;
            let b2_inv_mass = 1.0 / b2.mass;

            println!("e={e}\n");

            // Calculate impulse magnitude required to resolve collision
            let m = (v_norm * -(e + 1.0))/(b1_inv_mass + b2_inv_mass);
            let mut impulse = n * m;
            println!("m={m}\nimpulse={:?}", impulse);
            // Calculate positional correction
            let corrected_overlap = if overlap - SLOP > 0.0 { overlap - SLOP } else { 0.0 };
            let mut correction = n * (v2!(corrected_overlap) / ((b1_inv_mass + b2_inv_mass) * CORRECTION_PERCENTAGE));

            // Calculate tangent from normal
            // x = total - y
            let t = (rel_v - n * Vector2::dot(rel_v, n)).norm();
            println!("t={:?}\nrv.t={}", t, Vector2::dot(rel_v, t));
            // Find frictional coefficients
            let ks = 0.08;
            let kd = 0.056;

            let frictional_force = (-Vector2::dot(rel_v, t) / (b1_inv_mass + b2_inv_mass));
            println!("f_force={:?}", frictional_force);
            let mut friction;
            // If the frictional force below mew*R, apply it
            if frictional_force.abs() < ks * m {
                println!("Static");
                friction = t * frictional_force;
            // Else, the body is already in motion, and you should use
            } else {
                println!("Dynamic");
                friction = t * -m * kd;
            }
            println!("friction={:?}", friction);

            // Account for a frozen body not separating
            if b1.frozen || b2.frozen {
                impulse = impulse * 2.0;
                friction = friction * 2.0;
                correction = correction * 2.0;
            }

            // Apply impulse resolution & positional correction
            if !b1.frozen {
                b1.velocity = b1.velocity - (impulse * b1_inv_mass);
                b1.velocity = b1.velocity - (friction * b1_inv_mass);
                b1.position = b1.position - correction * b1_inv_mass;
            }
            if !b2.frozen {
                b2.velocity = b2.velocity + (impulse * b2_inv_mass);
                b2.velocity = b2.velocity + (friction * b2_inv_mass);
                b2.position = b2.position + correction * b2_inv_mass;
            }

            println!("====");
        }
    }
}
