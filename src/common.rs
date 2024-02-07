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
use num::Num;

use crate::app::objects::Body;

/* -------------------- VARIABLES ------------------- */
// General
pub type Disp = i32;
pub type Crd = f64;
pub type TSharedRef = Rc<RefCell<Shared>>;

// Collision
pub const GRID_SIZE: Vector2<usize> = crate::v2!(20, 20);
pub const PRECISION: i32 = 6;
pub type TBodyRef = Rc<RefCell<Body>>;
pub type TCollisionGrid = Vec<Vec<Vec<TBodyRef>>>;
pub type TCollisionPairs = Vec<[TBodyRef; 2]>;

/* ------------------- STRUCTURES ------------------- */
pub struct Shared {
    pub window_size: Vector2<u32>,
    pub collision_grid: TCollisionGrid,
    pub broad_phase_pairs: TCollisionPairs,
    pub narrow_phase_pairs: TCollisionPairs,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl<T: Copy + Num + AsPrimitive<f64> + AsPrimitive<Disp>> Vector2<T> {
    pub fn from(value: T) -> Vector2<T> {
        Vector2 {
            x: value,
            y: value,
        }
    }
    pub fn mag(self) -> f64 {
        let v: Vector2<f64> = self.to();
        let v2 = v * v;
        (v2.x + v2.y).sqrt()
    }
    pub fn norm(self) -> Vector2<f64> {
        let v: Vector2<f64> = self.to();
        let n = self.mag();

        Vector2 {
            x: v.x / n,
            y: v.y / n,
        }
    }
    pub fn disp(self) -> Vector2<Disp> {
        self.to()
    }
    pub fn dot(v1: Vector2<T>, v2: Vector2<T>) -> T {
        return (v1.x * v2.x) + (v1.y * v2.y);
    }
    pub fn project(v1: Vector2<T>, v2: Vector2<T>) -> Vector2<f64> {
        let v1: Vector2<f64> = v1.to();
        let v2: Vector2<f64> = v2.to();
        let dot = Vector2::dot(v1, v2);

        return v2 * (dot / v2.mag().powi(2));
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
    // Shorthand for: Vector2::from(_)
    ($n:expr) => {
        Vector2::from($n)
    };

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
pub fn round(n: f64) -> f64 {
    return (n * 10f64.powi(PRECISION)).round() / 10f64.powi(PRECISION)
}
