/*
    App.rs
    ----------------------------------------
    Description:
    * Central file for the physics engine
    * Provides the App struct itself, and manages all internal components of the physics engine
      (rendering, computation, etc.)
    * Manages the 'main loop'
 */
/* --------------------- IMPORTS -------------------- */
// Modules
pub mod engine;
pub mod video;
pub mod objects;
pub mod collision;

// Crates
use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::app::objects::Collection;
use sdl2::Sdl;
use sdl2::video::Window;
use crate::app::engine::Engine;
use crate::app::video::Video;
use crate::common::{MutShared, Vector2};
use crate::v2;

/* -------------------- VARIABLES ------------------- */
const DEBUG: bool = true;

/* ------------------- STRUCTURES ------------------- */
pub struct App<'a> {
    shared: MutShared,

    name: &'a str,
    size: Vector2<u32>,

    fps: u64,
    delta: u64,

    sdl2_ctx: Sdl,
    video: Video,
    engine: Engine<'a>,
    pub world: Collection<'a>,
}

/* -------------------- FUNCTIONS ------------------- */
impl<'a> App<'a> {
    pub fn new(name: &'a str, width: u32, height: u32) -> Self {
        let sdl2_ctx = sdl2::init().unwrap();

        let video = Video::new(&sdl2_ctx, name, width, height);
        let engine = Engine::new(video.shared.clone());

        let fps = 1000/60;
        let delta = fps;

        App {
            shared: video.shared.clone(),

            name,
            size: v2!(width, height),

            fps,
            delta,

            sdl2_ctx,
            video,
            engine,
            world: Collection::new("World"),
        }
    }

    pub fn start(&mut self) {
        // Preliminary setup
        self.video.canvas.clear();

        // self.video.point(v2!(100, 100), None);
        // self.video.canvas.present();

        let mut event_pump = self.sdl2_ctx.event_pump().unwrap();
        let mut stepped: bool = false;

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'main_loop; }
                    _ => {}
                }
            }

            if !(DEBUG && stepped) {
                stepped = true;

                self.video.canvas.clear();

                // Draw objects in world collection
                for body in self.world.bodies() {
                    self.video.draw_body(body);
                }

                // Update physics
                self.engine.step(&mut self.world, self.delta);

                self.video.canvas.present();
            }

            thread::sleep(Duration::from_millis(self.fps));
        }
    }

    /* --------------------- GETTERS -------------------- */
    fn window(&self) -> &Window {
        self.video.canvas.window()
    }
}