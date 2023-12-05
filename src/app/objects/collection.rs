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
pub struct Collection<'a> {
    name: &'a str,

    bodies: Vec<Body>,
    collections: Vec<Self>
}

enum CAddInput<'a> {
    Body(Body),
    Bodies(Vec<Body>),
    Collection(Collection<'a>),
    Collections(Vec<Collection<'a>>),
}


/* -------------------- FUNCTIONS ------------------- */
impl Collection {
    pub fn new(name: Option<&str>) -> Self {
        Collection {
            name: name.unwrap_or("New Collection"),
            bodies: vec![],
            collections: vec![],
        }
    }
    pub fn add(&mut self, object: CAddInput) {
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
}