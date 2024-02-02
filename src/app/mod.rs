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
pub mod ssm;

// Crates
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::video::Window;

use ssm::SystemStateManager;

use crate::app::engine::Engine;
use crate::app::objects::Body;
use crate::app::video::Video;
use crate::common::{TSharedRef, Vector2};
use crate::v2;

/* -------------------- VARIABLES ------------------- */
const DEBUG: bool = false;

/* ------------------- STRUCTURES ------------------- */
pub struct App {
    shared: TSharedRef,
    size: Vector2<u32>,

    fps: u64,
    delta: u64,

    sdl2_ctx: Sdl,
    video: Video,
    engine: Engine,
    system_state_manager: SystemStateManager,
}

/* -------------------- FUNCTIONS ------------------- */
impl App {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        let sdl2_ctx = sdl2::init().unwrap();

        let video = Video::new(&sdl2_ctx, name, width, height);
        let engine = Engine::new(video.shared.clone());

        let fps = 1000/60;
        let delta = fps;

        App {
            shared: video.shared.clone(),

            size: v2!(width, height),

            fps,
            delta,

            sdl2_ctx,
            video,
            engine,
            system_state_manager: SystemStateManager::new(),
        }
    }

    pub fn start(&mut self) {
        // Preliminary setup
        self.video.canvas.clear();

        // self.video.point(v2!(100, 100), None);
        // self.video.canvas.present();

        let mut event_pump = self.sdl2_ctx.event_pump().unwrap();
        let mut stepped: bool = false;

        let mut window_size = self.shared.borrow_mut().window_size;

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'main_loop; }
                    _ => {}
                }
            }

            // Update window size
            let cur_window_size = v2!(self.window().size().0, self.window().size().1);
            if window_size != cur_window_size {
                window_size = cur_window_size;
                self.shared.borrow_mut().window_size = cur_window_size;
            }

            if !(DEBUG && stepped) {
                stepped = true;

                self.video.pre_draw();

                // Draw objects in world collection
                for body_ref in self.system_state_manager.bodies() {
                    self.video.draw_body(body_ref);
                }

                // Update physics
                self.engine.step(self.system_state_manager.bodies(), self.delta);

                // Apply changes
                self.video.canvas.present();
            }

            thread::sleep(Duration::from_millis(self.fps));
        }
    }

    /* --------------------- GETTERS -------------------- */
    fn window(&self) -> &Window {
        self.video.canvas.window()
    }

    /* --------------------- SETTERS -------------------- */
    pub fn add_body(&mut self, body: Body) {
        self.system_state_manager.add_body(body);
    }
    pub fn add_bodies(&mut self, bodies: Vec<Body>) {
        self.system_state_manager.add_bodies(bodies);
    }
}
