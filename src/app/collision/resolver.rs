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
const CORRECTION_PERCENTAGE: f64 = 0.4;
const SLOP: f64 = 0.05;

impl CollisionResolver {
    pub fn new(shared: TSharedRef) -> Self {
        CollisionResolver {
            shared
        }
    }

    pub fn resolve(&self, collisions: Vec<CollisionResult>) {
        for result in collisions {
            // Deconstruct collision pair info
            let mut b1 = result.bodies[0].borrow_mut();
            let mut b2 = result.bodies[1].borrow_mut();
            let overlap = result.overlap;
            let n = result.normal;

            println!("b1.g={:?}", b1.velocity);
            println!("b2.g={:?}", b2.velocity);

            // Evaluate the bodies' positions
            if Vector2::dot(b1.globalise(v2!(0.0)), n) >= Vector2::dot(b2.globalise(v2!(0.0)), n) {
                (b1, b2) = (b2, b1);
            }

            // Body constants
            let e = if b1.restitution <= b2.restitution { b1.restitution } else { b2.restitution };
            // Relative contact points
            let r_1 = result.point - b1.center();
            let r_2 = result.point - b2.center();
            let r_1p = v2!(-1.0 * r_1.y, r_1.x);
            let r_2p = v2!(-1.0 * r_2.y, r_2.x);

            // Pre-resolution setup
            let rel_v = (b2.velocity + r_2p * b2.angular_velocity) - (b1.velocity + r_1p * b1.angular_velocity);
            let v_n = Vector2::dot(rel_v, n);
            // Tangent vector
            let mut t = rel_v - n * Vector2::dot(rel_v, n); // Calculate tangent vector
            let v_t = Vector2::dot(rel_v, t);

            // Skip resolution if bodies moving apart
            if v_n > 0.0 {
                return;
            }

            // Calculate impulse required to resolve collision
            let r_1pn = Vector2::dot(r_1p, n);
            let r_2pn = Vector2::dot(r_2p, n);

            // println!("r_1={:?}", (r_1pd * r_1pd) * b1.inv_inertia());

            let j = (v_n * -(e + 1.0)) /
                (b1.inv_mass() + b1.inv_mass()     +
                (r_1pn * r_1pn) * b1.inv_inertia() +
                (r_2pn * r_2pn) * b2.inv_inertia() );
            let mut impulse = n * j;

            // Find friction
            let mut f_impulse;

            t = t.norm();

            let ks = 0.12;  // Static frictional coefficient
            let kd = 0.08; // Dynamic frictional coefficient

            let r_1pt = Vector2::dot(r_1p, t);
            let r_2pt = Vector2::dot(r_2p, t);

            let f_j =  (v_t * -(e + 1.0)) /
                (b1.inv_mass() + b1.inv_mass()     +
                    (r_1pt * r_1pt) * b1.inv_inertia() +
                    (r_2pt * r_2pt) * b2.inv_inertia() );

            // If the frictional force below mew*R, apply it
            if f_j.abs() < ks * j {
                println!("Static");
                f_impulse = t * f_j;
                // Else, the body is already in motion, and you should use
            } else {
                println!("Dynamic");
                f_impulse = t * -j * kd;
            }

            // Calculate positional correction
            let mut correction = n * (if overlap - SLOP > 0.0 { overlap - SLOP } else { 0.0 }) * CORRECTION_PERCENTAGE;

            // Account for a frozen body not separating
            if b1.frozen || b2.frozen {
                impulse = impulse * 2.0;
                f_impulse = f_impulse * 2.0;
                correction = correction * 2.0;
            }

            // Apply impulse resolution & positional correction
            if !b1.frozen {
                b1.position = b1.position - correction * b1.inv_mass();
                b1.velocity = b1.velocity - (impulse + f_impulse) * b1.inv_mass();
                b1.angular_velocity = b1.angular_velocity - (Vector2::cross(r_1, impulse) + Vector2::cross(r_1, f_impulse)) * b1.inv_inertia();
            }
            if !b2.frozen {
                b2.position = b2.position + correction * b1.inv_mass();
                b2.velocity = b2.velocity + (impulse + f_impulse) * b1.inv_mass();
                b2.angular_velocity = b2.angular_velocity + (Vector2::cross(r_2, impulse) + Vector2::cross(r_2, f_impulse)) * b2.inv_inertia();
            }

            // println!("====");
        }
    }
}