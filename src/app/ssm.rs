/*
    ssm
    ----------------------------------------
    Description:
    * Provides access to folder-like structure for the physics engine
    * Separates different object types (Body, Collection, etc.) into separate vectors
    * TODO: Create hierarchy/directory-like structure to keep track of object groups
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::objects::Body;
use crate::common::TBodyRef;

/* ------------------- STRUCTURES ------------------- */
#[derive(Debug, PartialEq)]
pub struct SystemStateManager {
    bodies: Vec<TBodyRef>,
    // collections: Vec<Self>
}

/* -------------------- FUNCTIONS ------------------- */
impl SystemStateManager {
    pub fn new() -> Self {
        SystemStateManager {
            bodies: Vec::new(),
            // collections: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        let body_ref: TBodyRef = Rc::from(RefCell::from(body));

        if self.bodies.contains(&body_ref) {
            return;
        }

        self.bodies.push(body_ref);
    }

    pub fn add_bodies(&mut self, bodies: Vec<Body>) {
        for body in bodies {
            let body_ref: TBodyRef = Rc::from(RefCell::from(body));

            if self.bodies.contains(&body_ref) {
                continue;
            }

            self.bodies.push(body_ref);
        }
    }

    // pub fn contains_body(&self, body: Body) {
    //     let body_ref =
    // }

    /* --------------------- GETTERS -------------------- */
    pub fn bodies(&self) -> &Vec<TBodyRef> {
        return &self.bodies;
    }
}