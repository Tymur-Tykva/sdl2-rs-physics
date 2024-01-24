/*
    video.rs
    ----------------------------------------
    Description:
    * Handles the render step of the simulation
    * Defines how each object should be drawn based on internal parameters/overall configuration
 */
/* --------------------- IMPORTS -------------------- */
// Crates
use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::app::objects::Body;
use crate::common::{Crd, Vector2};
use crate::v2;

/* -------------------- VARIABLES ------------------- */
const POINT_SIZE: u32 = 4;


/* ------------------- STRUCTURES ------------------- */
pub struct Video {
    pub subsys: VideoSubsystem,
    pub canvas: WindowCanvas,

    points: bool,
    wireframe: bool,
    // pub window: Windows
}

/* -------------------- FUNCTIONS ------------------- */
impl Video {
    pub fn new(sdl2_ctx: &Sdl, name: &str, width: u32, height: u32) -> Self {
        let mut subsys = sdl2_ctx.video().unwrap();

        let mut window = subsys.window(name, width, height)
            .position_centered()
            .build().unwrap();

        let mut canvas = window.into_canvas()
            .present_vsync()
            .build().unwrap();

        Video {
            subsys,
            canvas,
            // window,

            points: true,
            wireframe: false,
        }
    }

    pub fn point(&mut self, c1: Vector2<Crd>, color_override: Option<Color>) {
        // println!("Point, {x} : {y}");
        let color = self.canvas.draw_color();

        self.canvas.set_draw_color(color_override.unwrap_or(Color::YELLOW));
        self.canvas.fill_rect(Rect::new(c1.x - (POINT_SIZE / 2) as i32, c1.y - (POINT_SIZE / 2) as i32, POINT_SIZE, POINT_SIZE)).unwrap();
        self.canvas.set_draw_color(color);
        // self.canvas.present();
    }

    pub fn line(&mut self, c1: Vector2<Crd>, c2: Vector2<Crd>, color_override: Option<Color>) {
        let color = self.canvas.draw_color();

        self.canvas.set_draw_color(color_override.unwrap_or(Color::WHITE));
        self.canvas.draw_line(Point::from((c1.x, c1.y)), Point::from((c2.x, c2.y))).unwrap();
        self.canvas.set_draw_color(color);
        // self.canvas.present();
    }

    pub fn draw_body(&mut self, body: &Body) {
        let vertices = body.vertices();

        // Draw internal lines
        // Dependent on: self.wireframe == true
        if self.wireframe {
            for i in 0..(vertices.len() - 1) {
                for j in i..vertices.len() {
                    self.line(
                        body.globalise(body.vertices()[i].to_vec2()),
                        body.globalise(body.vertices()[j].to_vec2()),
                        Some(Color::GREY)
                    )
                }
            }
        }

        // Draw external lines
        self.line(
            body.globalise(vertices[0].to_vec2()),
            body.globalise(vertices[vertices.len() - 1].to_vec2()),
            None
        );

        for i in 0..(vertices.len() - 1) {
            self.line(
                body.globalise(vertices[i].to_vec2()),
                body.globalise(vertices[i + 1].to_vec2()),
                None
            );
        }

        // Draw points
        // Dependent on: self.points == true
        if self.points {
            for i in 0..vertices.len() {
                self.point(body.globalise(vertices[i].to_vec2()), None);
            }
            // Origin
            self.point(body.globalise(v2!(0, 0)), Some(Color::BLUE));
        }
    }

    /* --------------------- GETTERS -------------------- */
    pub fn window(&self) -> &Window {
        self.canvas.window()
    }
}