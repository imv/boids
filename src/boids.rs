extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate cgmath;
extern crate shader_version;

use std::num::{abs, Zero};
use rand::{Rand, XorShiftRng};
use rand::distributions::{Normal, IndependentSample};
use piston::{
    WindowSettings,
    Render, Update,
    EventIterator,
    EventSettings
};
use graphics::{
    Context,
    AddColor, AddEllipse,
    Draw
};
use sdl2_game_window::WindowSDL2;
use opengl_graphics::Gl;
use cgmath::{
    ApproxEq,
    Point, Point2,
    Vector, EuclideanVector, Vector2
};
use shader_version::opengl::OpenGL_3_2;

pub struct Boid {
    pos: Point2<f64>,
    vel: Vector2<f64>,
}

pub static BOID_COUNT: uint = 50;
pub static BOID_RADIUS: f64 = 0.01;
pub static VISION_ANGLE: f64 = 2.0;
pub static CROWD_RADIUS: f64 = 0.1;
pub static FLOCK_RADIUS: f64 = 0.2;
pub static ACCEL_FACTOR: f64 = 10.0;
pub static ALIGN_FACTOR: f64 = 0.5;
pub static ALONE_VEL: f64 = 0.5;
pub static KEEP_VEL_FACTOR: f64 = 10.0;

pub struct App {
    boids: Vec<Boid>,
}

impl App {
    fn new() -> App {
        let mut rng = XorShiftRng::new_unseeded();
        let normal = Normal::new(0.0, ALONE_VEL*0.5f64.sqrt());
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

    fn update_pos(&mut self, dt: f64) {
        for b in self.boids.mut_iter() {
            b.pos.add_self_v(&b.vel.mul_s(dt))
        }
    }

    fn update_vel(&mut self, dt: f64) {
        fn towards_0_1(x: f64) -> f64 {
            if x < 0.0 {
                1.0
            } else if x > 1.0 {
                -1.0
            } else {
                0.0
            }
        }
        let accels: Vec<Vector2<f64>> = self.boids.iter().map(|b1| {
            let flocking = self.boids.iter()
                .filter(|b2|
                    !b2.pos.approx_eq(&b1.pos)
                ).filter(|b2| {
                    let dist_v = b2.pos.sub_p(&b1.pos);
                    // within vision angle
                    abs(b1.vel.angle(&dist_v).s) <= VISION_ANGLE &&
                    // within flocking distance
                    dist_v.length2() <= FLOCK_RADIUS*FLOCK_RADIUS
                }).fold(Zero::zero(), |accel: Vector2<f64>, b2| {
                    let dist_v = b2.pos.sub_p(&b1.pos);
                    accel
                        // cohesion & separation
                        + (dist_v - dist_v.normalize_to(CROWD_RADIUS))
                            .mul_s(ACCEL_FACTOR)
                        // alignment
                        + (b2.vel - b1.vel).mul_s(ALIGN_FACTOR)
                });
            let keep_vel = (b1.vel.normalize_to(ALONE_VEL) - b1.vel)
                .mul_s(KEEP_VEL_FACTOR);
            let steer_to_screen = Vector2 {
                x: towards_0_1(b1.pos.x),
                y: towards_0_1(b1.pos.y)
            };
            flocking + keep_vel + steer_to_screen
        }).collect();
        for (ref mut b, ref accel) in self.boids.mut_iter().zip(accels.iter()) {
            b.vel.add_self_v(&accel.mul_s(dt))
        }
    }
}

fn main() {
    let mut window = WindowSDL2::new(
        OpenGL_3_2, 
        WindowSettings {
            title: "Boids".to_string(),
            size: [640, 640],
            ..WindowSettings::default()
        }
    );
    let mut gl = Gl::new();
    let mut app = App::new();
    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };
    for e in EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(ref args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                app.render(&mut gl);
            },
            Update(ref args) => {
                app.update_pos(args.dt);
                app.update_vel(args.dt);
            },
            _ => {},
        }
    }
}
