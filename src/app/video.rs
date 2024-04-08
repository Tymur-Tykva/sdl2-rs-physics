/*
    video.rs
    ----------------------------------------
    Description:
    * Handles the render step of the simulation
    * Defines how each object should be drawn based on internal parameters/overall configuration
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use std::cell::RefCell;
use std::rc::Rc;

use sdl2::gfx::*;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::common::{ConvertPrimitives, Disp, GRID_SIZE, Shared, TBodyRef, TSharedRef, Vector2, Colors};
use crate::v2;

/* -------------------- VARIABLES ------------------- */
const POINT_SIZE: u32 = 4;

/* ------------------- STRUCTURES ------------------- */
pub struct Video {
    pub shared: TSharedRef,

    pub subsys: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub colors: Colors,

    aabb: bool,
    grid: bool,
    points: bool,
    wireframe: bool,
    collision_indicator: bool,
    // pub window: Windows
}

/* -------------------- FUNCTIONS ------------------- */
impl<'a> Video {
    pub fn new(sdl2_ctx: &Sdl, name: &str, width: u32, height: u32) -> Self {
        let subsys = sdl2_ctx.video().unwrap();

        let window = subsys.window(name, width, height)
            .position_centered()
            .build().unwrap();

        let canvas = window.into_canvas()
            .present_vsync()
            .build().unwrap();

        // Define shared variables
        let window_size = canvas.window().size();
        let shared = Rc::from(RefCell::from(Shared {
            window_size: v2!(window_size.0, window_size.1),
            collision_grid: Vec::new(),
            broad_phase_pairs: Vec::new(),
            narrow_phase_pairs: Vec::new(),
        }));

        Video {
            shared,

            subsys,
            canvas,
            colors: Colors,
            // window,

            aabb: false,
            grid: false,
            points: true,
            wireframe: false,
            collision_indicator: false,
        }
    }

    pub fn point(&mut self, c: Vector2<Disp>, color: Color) {
        let cached_color = self.canvas.draw_color();
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(c.x - (POINT_SIZE / 2) as Disp, c.y - (POINT_SIZE / 2) as Disp, POINT_SIZE, POINT_SIZE)).unwrap();
        self.canvas.set_draw_color(cached_color);
    }

    pub fn line(&mut self, c1: Vector2<Disp>, c2: Vector2<Disp>, color: Color) {
        let cached_color = self.canvas.draw_color();

        self.canvas.set_draw_color(color);
        self.canvas.draw_line(Point::from((c1.x, c1.y)), Point::from((c2.x, c2.y))).unwrap();
        self.canvas.set_draw_color(cached_color);
    }

    pub fn draw_body(&mut self, body_ref: &TBodyRef) {
        let body = body_ref.borrow_mut();
        let vertices = &body.vertices;

        // Draw AABB
        if self.aabb {
            let points: Vec<Vector2<Disp>> = body.aabb().points.iter().map(|p| p.disp()).collect();
            let x: Vec<i16> = points.clone().iter().map(|p| p.x as i16).collect();
            let y: Vec<i16> = points.clone().iter().map(|p| p.y as i16).collect();

            let draw_color = self.canvas.draw_color();
            self.canvas.aa_polygon(x.as_slice(), y.as_slice(), Colors::AC3).unwrap();
            self.canvas.set_draw_color(draw_color);
        }

        let mut x: Vec<i16> = vertices.clone().iter().map(|vtx| body.globalise(vtx.to_vec2()).x as i16).collect();
        let mut y: Vec<i16> = vertices.clone().iter().map(|vtx| body.globalise(vtx.to_vec2()).y as i16).collect();

        let draw_color = self.canvas.draw_color();
        self.canvas.filled_polygon(x.as_slice(), y.as_slice(), Colors::AC1).unwrap();
        self.canvas.aa_polygon(x.as_slice(), y.as_slice(), Colors::AC0).unwrap();
        self.canvas.set_draw_color(draw_color);

        // Draw points
        // Dependent on: self.points == true
        if self.points {
            for i in 0..vertices.len() {
                self.point(body.globalise(vertices[i].to_vec2()).disp(), Colors::AC2);
            }
            // Origin
            self.point(body.globalise(v2!(0.0)).disp(), Colors::AC3);
        }
    }

    pub fn pre_draw(&mut self) {
        self.canvas.clear();
        // TODO: Add bg color

        let window_size: Vector2<Disp> = self.shared.borrow_mut().window_size.to();
        let scalar: Vector2<Disp> = (window_size / GRID_SIZE.clone().to()).to();

        // Draw collision grid
        if self.grid {
            // Draw body grid collision underlay
            let collision_grid = self.shared.borrow_mut().collision_grid.clone();
            let scale_max = collision_grid.iter() // This gets the max length of a 3D vector
                .map(|v2| v2.iter()
                    .map(|v| v.len())
                    .max().unwrap_or(1)
                ).max().unwrap_or(1);

            if !(collision_grid.is_empty()) {
                for i in 0..GRID_SIZE.x {
                    for j in 0..GRID_SIZE.y {
                        let cell = &collision_grid[i][j];

                        if cell.len() > 0 {
                            let i = i as Disp;
                            let j = j as Disp;

                            let rect = Rect::new(j*scalar.x, i*scalar.y, scalar.to().x, scalar.to().y);
                            let cached_color = self.canvas.draw_color();

                            let scaled = cell.len() as f64 / scale_max as f64;
                            let r = 0;
                            let g = (150f64 * scaled) as u8;
                            let b = (50f64 * scaled) as u8;

                            self.canvas.set_draw_color(Color::RGB(r, g, b));
                            self.canvas.fill_rect(rect).unwrap();
                            self.canvas.set_draw_color(cached_color);
                        }
                    }
                }
            }

            // Draw grid
            let color = Colors::AC0;
            for i in 0..=GRID_SIZE.x {
                let i = i as Disp;
                self.line(v2!(i * scalar.x, 0), v2!(i * scalar.x, window_size.y), color);
            }

            for j in 0..=GRID_SIZE.y {
                let j = j as Disp;
                self.line(v2!(0, j * scalar.y), v2!(window_size.x, j * scalar.y), color);
            }
        }

        if self.collision_indicator {
            let broad_phase_pairs = self.shared.borrow().broad_phase_pairs.clone();
            let narrow_phase_pairs = self.shared.borrow().narrow_phase_pairs.clone();

            // println!("{:?}", broad_phase_pairs);
            // println!("{:?}", narrow_phase_pairs);

            for pair in broad_phase_pairs {
                let b1 = pair[0].borrow_mut();
                let b2 = pair[1].borrow_mut();

                self.line(b1.globalise(v2!(0.0)).disp(), b2.globalise(v2!(0.0)).disp(), Color::RGB(255, 165, 0));
            }

            for pair in narrow_phase_pairs {
                let b1 = pair.bodies[0].borrow_mut();
                let b2 = pair.bodies[1].borrow_mut();

                self.line(b1.globalise(v2!(0.0)).disp(), b2.globalise(v2!(0.0)).disp(), Colors::AC2);

                for i in 0..pair.contacts.len() {
                    self.point(pair.contacts[i].clone().to(), Color::CYAN);
                }

                // println!("o={}", pair.overlap);

                self.point(b1.center().to(), Color::YELLOW);
                self.point(b2.center().to(), Color::YELLOW);
            }

        }
    }

    /* --------------------- GETTERS -------------------- */
    pub fn window(&self) -> &Window {
        self.canvas.window()
    }
}