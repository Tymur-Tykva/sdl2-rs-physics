/*
    collection.rs
    ----------------------------------------
    Description:
    * Provides access to folder-like structure for the physics engine
    * Separates different object types (Body, Collection, etc.) into separate vectors
    * TODO: Provides ability to search through internal vectors
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use crate::app::objects::Body;


/* ------------------- STRUCTURES ------------------- */
#[derive(Debug, PartialEq)]
pub struct Collection<'a> {
    name: &'a str,

    bodies: Vec<Body>,
    collections: Vec<Self>
}

pub enum CAddInput<'a> {
    Body(Body),
    Bodies(Vec<Body>),
    Collection(Collection<'a>),
    Collections(Vec<Collection<'a>>),
}


/* -------------------- FUNCTIONS ------------------- */
impl<'a> Collection<'a> {
    pub fn new(name: &'a str) -> Self {
        Collection {
            name,
            bodies: Vec::new(),
            collections: Vec::new(),
        }
    }

    pub fn add(&mut self, object: CAddInput<'a>) {
        match object {
            CAddInput::Body(body) => {
                if self.bodies.contains(&body) { return; }
                self.bodies.push(body);
            },
            CAddInput::Bodies(bodies) => {
                for body in bodies {
                    if self.bodies.contains(&body) { continue; }
                    self.bodies.push(body);
                }
            }
            CAddInput::Collection(collection) => {
                if self.collections.contains(&collection) { return; }
                self.collections.push(collection);
            }
            CAddInput::Collections(collections) => {
                for collection in collections {
                    if self.collections.contains(&collection) { continue; }
                    self.collections.push(collection);
                }
            }
        }
    }

    /* --------------------- GETTERS -------------------- */
    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }
    pub fn mut_bodies(&mut self) -> &mut Vec<Body> { &mut self.bodies }
    pub fn collections(&self) -> &Vec<Collection> {
        &self.collections
    }
    pub fn mut_collections(& mut self) -> &mut Vec<Collection<'a>> {
        &mut self.collections
    }
}