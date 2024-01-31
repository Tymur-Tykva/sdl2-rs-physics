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
use std::ops::Index;
use std::rc::Rc;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::mouse::SystemCursor::No;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::common::{ConvertPrimitives, Crd, GRID_SIZE, Shared, TBodyRef, TSharedRef, Vector2};
use crate::v2;

/* -------------------- VARIABLES ------------------- */
const POINT_SIZE: u32 = 4;

/* ------------------- STRUCTURES ------------------- */
pub struct Video {
    pub shared: TSharedRef,

    pub subsys: VideoSubsystem,
    pub canvas: WindowCanvas,

    aabb: bool,
    grid: bool,
    points: bool,
    wireframe: bool,
    broad_phase_indicator: bool,
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
        }));

        Video {
            shared,

            subsys,
            canvas,
            // window,

            aabb: false,
            grid: false,
            points: true,
            wireframe: false,
            broad_phase_indicator: false,
        }
    }

    pub fn point(&mut self, c: Vector2<Crd>, color: Color) {
        let cached_color = self.canvas.draw_color();

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(c.x - (POINT_SIZE / 2) as i32, c.y - (POINT_SIZE / 2) as i32, POINT_SIZE, POINT_SIZE)).unwrap();
        self.canvas.set_draw_color(cached_color);
    }

    pub fn line(&mut self, c1: Vector2<Crd>, c2: Vector2<Crd>, color: Color) {
        let cached_color = self.canvas.draw_color();

        self.canvas.set_draw_color(color);
        self.canvas.draw_line(Point::from((c1.x, c1.y)), Point::from((c2.x, c2.y))).unwrap();
        self.canvas.set_draw_color(cached_color);
    }

    pub fn draw_body(&mut self, body_ref: &TBodyRef) {
        let body = body_ref.borrow_mut();
        let vertices = body.vertices();

        // Draw AABB
        if self.aabb {
            let points = body.aabb().points;

            self.line(points[0], points[3], Color::GREEN);
            for i in 0..(points.len() - 1) {
                self.line(points[i], points[i + 1], Color::GREEN);
            }
        }

        // Draw internal lines
        // Dependent on: self.wireframe == true
        if self.wireframe {
            for i in 0..(vertices.len() - 1) {
                for j in i..vertices.len() {
                    self.line(
                        body.globalise(body.vertices()[i].to_vec2()),
                        body.globalise(body.vertices()[j].to_vec2()),
                        Color::GREY
                    )
                }
            }
        }

        // Draw external lines
        self.line(
            body.globalise(vertices[0].to_vec2()),
            body.globalise(vertices[vertices.len() - 1].to_vec2()),
            Color::WHITE,
        );

        for i in 0..(vertices.len() - 1) {
            self.line(
                body.globalise(vertices[i].to_vec2()),
                body.globalise(vertices[i + 1].to_vec2()),
                Color::WHITE,
            );
        }

        // Draw points
        // Dependent on: self.points == true
        if self.points {
            for i in 0..vertices.len() {
                self.point(body.globalise(vertices[i].to_vec2()), Color::YELLOW);
            }
            // Origin
            self.point(body.globalise(v2!(0, 0)), Color::BLUE);
        }


        // TODO: REMOVE TEMPORARY DISPLAY
        let p1 = body.vertices()[0].to_vec2();
        let p2 = body.vertices()[1].to_vec2();
        let edge = p2 - p1;
        let normal = v2!(-edge.y, edge.x);

        self.point(body.globalise(p1), Color::CYAN);
        self.point(body.globalise(p2), Color::CYAN);
        self.point(body.globalise(edge), Color::MAGENTA);
        self.point(body.globalise(normal), Color::RED);
    }

    pub fn pre_draw(&mut self) {
        self.canvas.clear();
        // TODO: Add bg color

        let window_size: Vector2<Crd> = self.shared.borrow_mut().window_size.to();
        let scalar: Vector2<Crd> = (window_size / GRID_SIZE.clone().to()).to();

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
                            let i = i as Crd;
                            let j = j as Crd;

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
            let color = Color::RGB(50, 50, 50);
            for i in 0..=GRID_SIZE.x {
                let i = i as Crd;
                self.line(v2!(i * scalar.x, 0), v2!(i * scalar.x, window_size.y), color);
            }

            for j in 0..=GRID_SIZE.y {
                let j = j as Crd;
                self.line(v2!(0, j * scalar.y), v2!(window_size.x, j * scalar.y), color);
            }
        }

        if self.broad_phase_indicator {
            let broad_phase_pairs = self.shared.borrow().broad_phase_pairs.clone();

            for pair in broad_phase_pairs {
                let b1 = pair[0].borrow_mut();
                let b2 = pair[1].borrow_mut();

                self.line(b1.globalise(Vector2::from(0)), b2.globalise(Vector2::from(0)), Color::RGB(255, 165, 0));
            }
        }
    }

    /* --------------------- GETTERS -------------------- */
    pub fn window(&self) -> &Window {
        self.canvas.window()
    }
}