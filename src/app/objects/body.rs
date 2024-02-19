/*
    body.rs
    ----------------------------------------
    Description:Description
    * Provides struct for all physical objects that are rendered in the simulation
    * Internally tracks properties (position, velocity, etc.)
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use std::f64::consts::PI;

use crate::common::{AABB, BodyForm, ConvertPrimitives, Disp, Crd, Vector2, Vertex};
use crate::{v2, vtx};

/* -------------------- VARIABLES ------------------- */


/* ------------------- STRUCTURES ------------------- */
/// Internal struct for defining & updating bodies (any object which has a physical presence in the simulation).
#[derive(Debug, PartialEq)]
pub struct Body {
    // Internal
    pub form: BodyForm,
    pub position: Vector2<Crd>,
    pub prev_position: Vector2<Crd>,
    pub rotation: i64,
    pub origin: Vector2<Crd>,
    pub radius: Option<f64>,
    // BodyForm::Polygon
    pub sides: u32,
    pub vertices: Vec<Vertex>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    // Physics
    pub center: Vector2<f64>, // Center of mass; measured radius as f64 from origin
    pub frozen: bool, // Whether the body's forces shouldn't be updated at the physics step
    pub mass: f64, // Mass of the object, exerted at it's center of mass
    pub velocity: Vector2<f64>,
    pub restitution: f64,
    pub initial_friction: f64,
    pub continuous_friction: f64,
    pub angular_velocity: i32,
    pub air_friction: f64,
    pub force_buffer: Vector2<f64>, // Forces applied onto the body, pre Body::update()
}


/* -------------------- FUNCTIONS ------------------- */
impl Body {
    /// Constructor for the Body struct.
    pub fn new(
        form: BodyForm, position: Vector2<Disp>, radius: Option<f64>, // Internal properties
        sides: u32, width: Option<u32>, height: Option<u32>,          // Polygonal properties
        mass: f64, restitution: f64,                                  // Physics properties
    ) -> Self {
        let origin: Vector2<Crd>;
        let center: Vector2<f64>;
        let vertices: Vec<Vertex>;

        // If the body is a rect-like
        if sides == 4 && width.is_some() && height.is_some() {
            let width = width.unwrap_or(1)   as Crd;
            let height = height.unwrap_or(1) as Crd;

            origin = v2!(0).to();
            center = v2!(width/2.0, height/2.0).to();

            // Manually define initial vertex positions
            vertices = vec![
                vtx!(0, 0.0, 0.0     ),
                vtx!(1, 0.0, height  ),
                vtx!(2, width, height),
                vtx!(3, width, 0.0   ),
            ];
            // Standard polygon formation
        } else {
            let radius = radius.unwrap_or(1f64);
            origin = Vector2::from(radius).to();
            center = Vector2::from(radius).to();

            // Call vertex calculation
            vertices = Body::calculate_vertices(sides, radius);
        }

        Body {
            // Internal
            form,
            position: position.to(),
            prev_position: position.clone().to(),
            rotation: 0,
            origin,
            radius,
            // BodyForm::Polygon
            sides,
            vertices,
            width,
            height,
            // Physics
            mass,
            center,
            restitution,
            initial_friction: 0.9,
            continuous_friction: 0.7,
            frozen: false,
            velocity: v2!(0f64, 0f64),
            angular_velocity: 0,
            air_friction: 0.01,
            force_buffer: v2!(0f64, 0f64),
        }
    }

    /// Physics update for the body. Called every frame.
    pub fn update(&mut self, delta: u64) {
        if self.frozen { return; }

        let delta = delta as f64;
        let inv_mass = 1.0 / self.mass;

        self.velocity = self.velocity + (self.force_buffer * inv_mass) * delta;
        self.position = self.position + self.velocity * delta;
    }

    /// Evaluates whether the given Body object is a rect-like.
    /// Checks if it has 4 sides, and has its width & height properties defined.
    pub fn is_rect(&self) -> bool {
        self.form == BodyForm::Polygon && self.sides == 4 && self.width.is_some() && self.height.is_some()
    }

    /// Internal method for calculating the initial vertex position of a polygon.
    fn calculate_vertices(sides: u32, radius: f64) -> Vec<Vertex> {
        let a = (2f64 * PI) / sides as f64;
        let mut vertices = Vec::new();


        // Optimization for when number of sides is even
        if sides % 2 == 0 {
            for i in 1..=(sides - 2) / 2 {
                let j = i as f64;
                let x = (radius * (a * j).cos()) as Crd;
                let y = (radius * (a * j).sin()) as Crd;

                vertices.push(vtx!(i,       x, -y));
                vertices.push(vtx!(sides-i, x, y));
            }

            vertices.push(vtx!(0,       radius,  0.0));
            vertices.push(vtx!(sides/2, -radius, 0.0));
        }
        // Optimization for when number of sides is odd
        else {
            for i in 1..=(sides - 1) / 2 {
                let j = i as f64;
                let x = radius * (a * j).cos() as Crd;
                let y = radius * (a * j).sin() as Crd;

                vertices.push(vtx!(i,         x, -y));
                vertices.push(vtx!(sides - i, x, y ));
            }

            vertices.push(vtx!(0, radius as Crd, 0.0));
        }

        vertices.sort_by(|&a, &b| (a.id).cmp(&b.id));
        vertices
    }

    /// Convert local Vector2 (vector about the object's origin) into global space
    pub fn globalise(&self, vec: Vector2<Crd>) -> Vector2<Crd> {
        self.origin + self.position + vec.to()
    }

    /// Returns the axis-aligned bounding box of the object.
    pub fn aabb(&self) -> AABB {
        let points;

        if self.radius.is_some() {
            let r = self.radius.unwrap() as Crd;

            points = vec![v2!(-r, -r), v2!(r, -r), v2!(r, r), v2!(-r, r)]
                .iter().map(|&v2| self.globalise(v2)).collect();
        } else {
            let w = self.width.unwrap() as Crd;
            let h = self.height.unwrap() as Crd;

            points = vec![v2!(0.0, 0.0), v2!(0.0, h), v2!(w, h), v2!(w, 0.0)]
                .iter().map(|&v2| self.globalise(v2)).collect();
        }

        AABB {
            points
        }
    }

    pub fn axes(&self) -> Vec<Vector2<f64>> {
        let mut axes = Vec::new();

        for i in 0..self.sides as usize {
            let p1 = self.vertices[i].to_vec2();
            let p2 = self.vertices[if i + 1 == self.sides as usize { 0 } else { i + 1 }].to_vec2();

            let edge = p2 - p1;
            let normal = v2!(-edge.y, edge.x).norm();

            // println!("Axis={:?}", normal);
            // println!("Norm={:?}", normal.norm());

            axes.push(normal);
        }

        axes
    }

    /* --------------------- GETTERS -------------------- */
    // pub fn position(&self) -> Vector2<Crd>  {
    //     self.position
    // }
    // pub fn radius(&self) -> Option<f64>  {
    //     self.radius
    // }
    // pub fn vertices(&self) -> &Vec<Vertex> {
    //     &self.vertices
    // }
    // pub fn origin(&self) -> Vector2<Crd> {
    //     self.origin
    // }
    // pub fn sides(&self) ->u32 {
    //     self.sides
    // }
    // pub fn frozen(&self) -> bool {
    //     self.frozen
    // }
    // pub fn force_buffer(&self) -> Vector2<f64> {
    //     self.force_buffer
    // }
    // pub fn restitution(&self) -> f64 {
    //     self.restitution
    // }
    // pub fn mass(&self) -> u32 {
    //     self.mass
    // }
    pub fn indent(&self) -> () {
        println!("- sides={:?}\n- w={:?}", self.sides, self.width.unwrap_or(0));
    }
    /* --------------------- SETTERS -------------------- */
    // pub fn set_frozen(&mut self, frozen: bool) { self.frozen = frozen }
    // pub fn set_position(&mut self, position: Vector2<Crd>) {
    //     self.position = position;
    // }
    // pub fn set_force_buffer(&mut self, force_buffer: Vector2<f64>) {
    //     self.force_buffer = force_buffer;
    // }
    pub fn clear_force_buffer(&mut self) {
        self.force_buffer = v2!(0f64, 0f64);
    }
    pub fn set_frozen(mut self, frozen: bool) -> Self {
        self.frozen = frozen;
        self
    }
}

/* --------------------- MACROS --------------------- */
#[macro_export]
macro_rules! poly {
    // Generate polygon with a mass of 1
    ($pos:expr, $radius:expr, $sides:expr) => {
        Body::new(BodyForm::Polygon, $pos, Some($radius as f64), $sides, None, None, 1.0, 0.7)
    };
    // Generate polygon with custom mass
    ($pos:expr, $radius:expr, $sides:expr, $mass:expr) => {
        Body::new(BodyForm::Polygon, $pos, Some($radius as f64), $sides, None, None, $mass, 0.7)
    };
}

#[macro_export]
macro_rules! rect {
    // Generate a rect-like with a mass of 1
    ($pos:expr, $width:expr, $height:expr) => {
        Body::new(BodyForm::Polygon, $pos, None, 4, Some($width), Some($height), 1.0, 0.7)
    };
    // Generate a rect-like with a custom mass
    ($pos:expr, $width:expr, $height:expr, $mass:expr) => {
        Body::new(BodyForm::Polygon, $pos, None, 4, Some($width), Some($height), $mass, 0.7)
    };
}