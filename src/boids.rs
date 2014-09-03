extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate shader_version;

use rand::{Rand, XorShiftRng};
use piston::{
    GameWindowSettings,
    Render,
    GameIterator,
    GameIteratorSettings,
    RenderArgs
};
use graphics::{Context, AddColor, Draw};
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
use cgmath::Point2;
use cgmath::Vector2;
use shader_version::opengl::OpenGL_3_2;

pub struct Boid {
    pos: Point2<f64>,
    vel: Vector2<f64>,
}

pub static BOID_COUNT: uint = 50;

pub struct App {
    gl: Gl,
    boids: Vec<Boid>,
}

impl App {
    fn new() -> App {
        let mut rng = XorShiftRng::new_unseeded();
        App {
            gl: Gl::new(),
            boids: Vec::from_fn(BOID_COUNT, |_| Boid {
                pos: Point2 { x: Rand::rand(&mut rng), y: Rand::rand(&mut rng) },
                vel: Vector2 { x: Rand::rand(&mut rng), y: Rand::rand(&mut rng) },
            })
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);
        let context = Context::abs(args.width as f64, args.height as f64);
        context
            .rgb(0.25, 0.5, 1.0)
            .draw(&mut self.gl);
    }
}

fn main() {
    let mut window = GameWindowSDL2::new(
        OpenGL_3_2, 
        GameWindowSettings {
            title: "Boids".to_string(),
            ..GameWindowSettings::default()
        }
    );
    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };
    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(ref args) => {
                app.render(args);
            },
            _ => {},
        }
    }
}
