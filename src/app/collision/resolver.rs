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

            // println!("== Resolution ==");
            // println!("b1");
            // b1.ident();
            // println!("b2");
            // b2.ident();
            // println!("====");
            // println!("normal={:?}\noverlap={overlap}\n", n);

            // Pre-resolution setup
            let rel_v = b2.velocity - b1.velocity;
            let v_norm = Vector2::dot(rel_v, n);

            // println!("rv={:?}\nv_norm={v_norm}", rel_v);

            let e = if b1.restitution <= b2.restitution { b1.restitution } else { b2.restitution };
            let b1_inv_mass = 1.0 / b1.mass;
            let b2_inv_mass = 1.0 / b2.mass;
            let b1_inv_inertia = 1.0 / b1.inertia;
            let b2_inv_inertia = 1.0 / b2.inertia;

            // println!("e={e}\n");

            // Calculate impulse magnitude required to resolve collision
            let mut r_1 = b1.center * b1.center + result.point * result.point;
            r_1 = v2!((r_1.x + r_1.y).sqrt(), (r_1.x + r_1.y).sqrt());
            let mut r_2 = b2.center * b2.center + result.point * result.point;
            r_2 = v2!((r_2.x + r_2.y).sqrt(), (r_2.x + r_2.y).sqrt());
            let b1_ang = Vector2::cross(v2!(Vector2::cross(r_1, n)), r_1) * b1_inv_inertia;
            let b2_ang = Vector2::cross(v2!(Vector2::cross(r_2, n)), r_2) * b2_inv_inertia;

            let m = (v_norm * -(e + 1.0))/(b1_inv_mass + b2_inv_mass + b1_ang + b2_ang);
            let mut impulse = n * m;
            let rot_impulse = impulse.clone();
            // println!("m={m}\nimpulse={:?}", impulse);

            // Calculate positional correction
            let corrected_overlap = if overlap - SLOP > 0.0 { overlap - SLOP } else { 0.0 };
            let mut correction = n * (v2!(corrected_overlap) / ((b1_inv_mass + b2_inv_mass) * CORRECTION_PERCENTAGE));

            // Calculate tangent from normal
            // x = total - y
            let t = (rel_v - n * Vector2::dot(rel_v, n)).norm();
            // println!("t={:?}\nrv.t={}", t, Vector2::dot(rel_v, t));

            // Find frictional coefficients
            let ks = 0.08;
            let kd = 0.056;

            let frictional_force = -Vector2::dot(rel_v, t) / (b1_inv_mass + b2_inv_mass + b1_ang + b2_ang);
            // println!("f_force={:?}", frictional_force);
            let mut friction;

            // If the frictional force below mew*R, apply it
            if frictional_force.abs() < ks * m {
                // println!("Static");
                friction = t * frictional_force;
            // Else, the body is already in motion, and you should use
            } else {
                // println!("Dynamic");
                friction = t * -m * kd;
            }
            // println!("friction={:?}", friction);

            // Account for a frozen body not separating
            if b1.frozen || b2.frozen {
                impulse = impulse * 2.0;
                friction = friction * 2.0;
                correction = correction * 2.0;
            }

            // Apply impulse resolution & positional correction
            if !b1.frozen {
                b1.position = b1.position - correction * b1_inv_mass;
                b1.velocity = b1.velocity - (impulse + friction) * b1_inv_mass;
                // b1.angular_velocity = b1.angular_velocity + (Vector2::cross(r_1, rot_impulse)) * b1_inv_inertia;
            }
            if !b2.frozen {
                b2.position = b2.position + correction * b2_inv_mass;
                b2.velocity = b2.velocity + (impulse + friction) * b2_inv_mass;
                // b2.angular_velocity = b2.angular_velocity - Vector2::cross(r_2, rot_impulse) * b2_inv_inertia;
            }

            // println!("====");
        }
    }
}
