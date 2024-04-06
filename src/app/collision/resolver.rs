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

            let points = result.contacts.len();

            // Ensure bodies always oriented in an expected manner
            if Vector2::dot(b1.globalise(v2!(0.0)), n) >= Vector2::dot(b2.globalise(v2!(0.0)), n) {
                (b1, b2) = (b2, b1);
            }

            // Body constants
            let e = if b1.restitution <= b2.restitution { b1.restitution } else { b2.restitution };
            let ks = 0.12; // Static frictional coefficient
            let kd = 0.04; // Dynamic frictional coefficient

            // Shared arrays
            let mut js: Vec<f64> = Vec::new();
            let mut impulses: Vec<Vector2<f64>> = Vec::new();
            let mut f_impulses: Vec<Vector2<f64>> = Vec::new();

            let mut r_1s: Vec<Vector2<f64>> = Vec::new();
            let mut r_2s: Vec<Vector2<f64>> = Vec::new();

            // Apply positional correction
            let correction = n * overlap * (if b1.frozen || b2.frozen { 1.0 } else { 0.5 }) * CORRECTION_PERCENTAGE / (points as f64);
            b1.position = b1.position - correction * b1.inv_mass();
            b2.position = b2.position + correction * b2.inv_mass();

            // Calculate normal impulse
            for i in 0..points {
                let point = result.contacts[i].clone();

                // Relative contact points
                let r_1 = point - b1.center();
                let r_2 = point - b2.center();
                let r_1p = v2!(-1.0 * r_1.y, r_1.x);
                let r_2p = v2!(-1.0 * r_2.y, r_2.x);

                // Pre-resolution setup
                let rel_v = (b2.velocity + r_2p * b2.angular_velocity) - (b1.velocity + r_1p * b1.angular_velocity);
                let v_n = Vector2::dot(rel_v, n);

                // Skip resolution if bodies moving apart
                // if v_n > 0.0 {
                //     continue;
                // }

                // Calculate impulse required to resolve collision
                let r_1pn = Vector2::dot(r_1p, n);
                let r_2pn = Vector2::dot(r_2p, n);

                let j = (v_n * -(e + 1.0)) /
                    (b1.inv_mass() + b2.inv_mass()     +
                    (r_1pn * r_1pn) * b1.inv_inertia() +
                    (r_2pn * r_2pn) * b2.inv_inertia()
                    ) / (points as f64);
                let impulse = n * j;

                // Store calculation results
                r_1s.push(r_1);
                r_2s.push(r_2);

                js.push(j);
                impulses.push(impulse);
            }

            // Apply normal impulse
            for i in 0..points {
                let r_1 = r_1s[i];
                let r_2 = r_2s[i];
                let impulse = impulses[i];

                // Apply linear impulse
                b1.velocity = b1.velocity - impulse * b1.inv_mass();
                b2.velocity = b2.velocity + impulse * b2.inv_mass();
                // Apply angular impulse
                b1.angular_velocity = b1.angular_velocity - Vector2::cross(r_1, impulse) * b1.inv_inertia();
                b2.angular_velocity = b2.angular_velocity + Vector2::cross(r_2, impulse) * b2.inv_inertia();
            }

            // Calculate frictional impulse
            for i in 0..points {
                let r_1 = r_1s[i];
                let r_2 = r_2s[i];
                let r_1p = v2!(-1.0 * r_1.y, r_1.x);
                let r_2p = v2!(-1.0 * r_2.y, r_2.x);

                let rel_v = (b2.velocity + r_2p * b2.angular_velocity) - (b1.velocity + r_1p * b1.angular_velocity);

                // Tangent vector
                let mut t = (rel_v - n * Vector2::dot(rel_v, n)).norm(); // Calculate tangent vector

                // Do not resolve friction if tangent impulse negligible
                // if Vector2::<f64>::almost_eq(t, v2!(0f64)) {
                //     continue;
                // } else {
                //     t = t.norm();
                // }

                let v_t = Vector2::dot(rel_v, t);

                // Find friction
                let r_1pt = Vector2::dot(r_1p, t);
                let r_2pt = Vector2::dot(r_2p, t);

                let f_j = -v_t /
                    (b1.inv_mass() + b2.inv_mass()     +
                    (r_1pt * r_1pt) * b1.inv_inertia() +
                    (r_2pt * r_2pt) * b2.inv_inertia()
                    ) / (points as f64);

                // If the frictional force below mew*R, apply it
                let f_impulse;
                let j = js[i];

                if f_j.abs() <= ks * j {
                    f_impulse = t * f_j;
                } else {
                    f_impulse = t * -j * kd;
                }

                f_impulses.push(f_impulse);
            }

            // Apply tangent impulse
            for i in 0..points {
                let r_1 = r_1s[i];
                let r_2 = r_2s[i];
                let f_impulse = f_impulses[i];

                // Apply linear impulse
                b1.velocity = b1.velocity - f_impulse * b1.inv_mass();
                b2.velocity = b2.velocity + f_impulse * b2.inv_mass();
                // Apply angular impulse
                b1.angular_velocity = b1.angular_velocity - Vector2::cross(r_1, f_impulse) * b1.inv_inertia();
                b2.angular_velocity = b2.angular_velocity + Vector2::cross(r_2, f_impulse) * b2.inv_inertia();
            }
        }
    }
}