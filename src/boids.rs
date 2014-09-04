extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate shader_version;

use rand::{Rand, XorShiftRng};
use rand::distributions::{Normal, IndependentSample};
use piston::{
    GameWindowSettings,
    Render, Update,
    GameIterator,
    GameIteratorSettings
};
use graphics::{
    Context,
    AddColor, AddEllipse,
    Draw
};
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
use cgmath::{
    Point, Point2,
    Vector, Vector2
};
use shader_version::opengl::OpenGL_3_2;

pub struct Boid {
    pos: Point2<f64>,
    vel: Vector2<f64>,
}

pub static BOID_COUNT: uint = 50;
pub static BOID_RADIUS: f64 = 0.01;

pub struct App {
    boids: Vec<Boid>,
}

impl App {
    fn new() -> App {
        let mut rng = XorShiftRng::new_unseeded();
        let normal = Normal::new(0.0, 0.5);
        App {
            boids: Vec::from_fn(BOID_COUNT, |_| Boid {
                pos: Point2 { x: Rand::rand(&mut rng), y: Rand::rand(&mut rng) },
                vel: Vector2 {
                    x: normal.ind_sample(&mut rng),
                    y: normal.ind_sample(&mut rng)
                }
            })
        }
    }

    fn render(&self, gl: &mut Gl) {
        let context = Context::abs(1.0, 1.0);
        context
            .rgb(0.25, 0.5, 1.0)
            .draw(gl);
        for b in self.boids.iter() {
            context
                .rgb(0.0, 0.0, 0.0)
                .circle(b.pos.x, b.pos.y, 0.01)
                .draw(gl);
        }
    }

    fn update(&mut self, dt: f64) {
        for b in self.boids.mut_iter() {
            b.pos.add_self_v(&b.vel.mul_s(dt))
        }
    }
}

fn main() {
    let mut window = GameWindowSDL2::new(
        OpenGL_3_2, 
        GameWindowSettings {
            title: "Boids".to_string(),
            size: [640, 640],
            ..GameWindowSettings::default()
        }
    );
    let mut gl = Gl::new();
    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };
    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(ref args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                app.render(&mut gl);
            },
            Update(ref args) =>
                app.update(args.dt),
            _ => {},
        }
    }
}
