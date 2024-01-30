/*
    common.rs
    ----------------------------------------
    Description:
    * Provides access to commonly-used functions, data types, etc. to the physics engine
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

use num::cast::AsPrimitive;

use crate::app::objects::Body;

/* -------------------- VARIABLES ------------------- */
// General
pub type Crd = i32;
pub type TSharedRef = Rc<RefCell<Shared>>;

// Collision
pub const GRID_SIZE: Vector2<usize> = crate::v2!(20, 20);
pub type TBodyRef = Rc<RefCell<Body>>;
pub type TCollisionGrid = Vec<Vec<Vec<TBodyRef>>>;
pub type TCollisionPairs = Vec<[TBodyRef; 2]>;

/* ------------------- STRUCTURES ------------------- */
pub struct Shared {
    pub window_size: Vector2<u32>,
    pub collision_grid: TCollisionGrid,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl<T: Copy> Vector2<T> {
    pub fn from(value: T) -> Vector2<T> {
        Vector2 {
            x: value,
            y: value,
        }
    }
}

// Conversion of Vector2 into a Vector2 of a different type; Vector2<A> -> Vector2<B>
pub trait ConvertPrimitives<T> {
    fn to<U>(self) -> Vector2<U>
        where
            T: AsPrimitive<U>, U: Copy + 'static
    ;
}
impl<T> ConvertPrimitives<T> for Vector2<T> {
    fn to<U>(self) -> Vector2<U>
        where
            T: AsPrimitive<U>, U: Copy + 'static
    {
        Vector2 {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }
}

// Mathematical operations for the Vector2 struct
impl<T: Add<Output=T>> Add<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T: Sub<Output=T>> Sub<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T: Div<Output=T>> Div<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl<T: Mul<Output=T>> Mul<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

// Mathematical operations between a Vector2 and a numerical
impl<T: Mul<Output=T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl<T: Div<Output=T> + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn div(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2M<T> {
    pub x: T,
    pub y: T,
    pub m: f64,
}
impl<T> Vector2M<T> {
    pub fn to_vec2(self) -> Vector2<T> {
        Vector2 { x: self.x, y: self.y }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vertex {
    pub id: u32,
    pub x: Crd,
    pub y: Crd,
}
impl Vertex {
    pub fn to_vec2(self) -> Vector2<Crd> {
        Vector2 { x: self.x, y: self.y }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BodyForm {
    Polygon,
    Circle
}

#[derive(Debug, Clone)]
pub struct AABB {
    pub points: Vec<Vector2<Crd>>,
}

/* --------------------- MACROS --------------------- */
#[macro_export]
macro_rules! v2 {
    // Shorthand for: Vector2 {}
    // e.g. v2!(16, 16)
    ($x:expr, $y:expr) => {
        Vector2 { x:$x, y:$y }
    };
    // Shorthand for Vector2M {}
    // e.g. v2!(16, 16, 0.01)
    ($x:expr, $y:expr, $m:expr) => {
        Vector2M { x:$x, y:$y, m:$m }
    };
}

#[macro_export]
macro_rules! vtx {
    // Shorthand for Vertex {}
    // e.g. vtx!(16, 16 : 0)
    ($id:expr, $x:expr, $y:expr) => {
        Vertex { id:$id, x:$x, y:$y }
    };
}

/* ------------------- FUNCTIONS ------------------- */

