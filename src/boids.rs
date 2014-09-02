extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate cgmath;

use rand::{Rand, XorShiftRng};
use piston::{Game, GameWindowSettings, GameIteratorSettings, RenderArgs};
use graphics::{Context, AddColor, Draw};
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
use cgmath::point::Point2;
use cgmath::vector::Vector2;

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
}

impl Game for App {
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
        GameWindowSettings {
            title: "Boids".to_string(),
            ..GameWindowSettings::default()
        }
    );
    let mut app = App::new();
    app.run(&mut window,
        &GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );
}
