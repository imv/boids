extern crate num;
extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate nalgebra as na;

use num::{abs, Zero};
use rand::{Rand, XorShiftRng};
use rand::distributions::{Normal, IndependentSample};
use piston::window::{WindowSettings, Size};
use piston::event::{Events, RenderEvent, UpdateEvent};
use graphics::{Context, Transformed};
use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};
use na::{
    ApproxEq, RotationTo, Norm, Axpy,
    Pnt2, Vec2
};

pub struct Boid {
    pos: Pnt2<f64>,
    vel: Vec2<f64>,
}

pub static BOID_COUNT: u32 = 50;
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
            boids: (0..BOID_COUNT).map(|_| Boid {
                pos: Pnt2 { x: Rand::rand(&mut rng), y: Rand::rand(&mut rng) },
                vel: Vec2 {
                    x: normal.ind_sample(&mut rng),
                    y: normal.ind_sample(&mut rng)
                }
            }).collect()
        }
    }

    fn render(&self, context: Context, gl: &mut GlGraphics) {
        graphics::clear([0.25, 0.5, 1.0, 1.0], gl);
        let context = context.scale(640.0, 640.0);
        for b in self.boids.iter() {
            graphics::ellipse([0.0, 0.0, 0.0, 1.0],
                graphics::ellipse::circle(b.pos.x, b.pos.y, 0.01),
                context.transform, gl);
        }
    }

    fn update_pos(&mut self, dt: f64) {
        for b in self.boids.iter_mut() {
            b.pos.axpy(&dt, b.vel.as_pnt())
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
        let accels: Vec<Vec2<f64>> = self.boids.iter().map(|b1| {
            let flocking = self.boids.iter()
                .filter(|b2|
                    !b2.pos.approx_eq(&b1.pos)
                ).filter(|b2| {
                    let dist_v = b2.pos - b1.pos;
                    // within vision angle
                    abs(b1.vel.angle_to(&dist_v)) <= VISION_ANGLE &&
                    // within flocking distance
                    dist_v.sqnorm() <= FLOCK_RADIUS*FLOCK_RADIUS
                }).fold(Zero::zero(), |accel: Vec2<f64>, b2| {
                    let dist_v = b2.pos - b1.pos;
                    accel
                        // cohesion & separation
                        + (dist_v - dist_v*(CROWD_RADIUS/dist_v.norm()))*ACCEL_FACTOR
                        // alignment
                        + (b2.vel - b1.vel)*ALIGN_FACTOR
                });
            let keep_vel = (b1.vel*(ALONE_VEL/b1.vel.norm()) - b1.vel)*KEEP_VEL_FACTOR;
            let steer_to_screen = Vec2 {
                x: towards_0_1(b1.pos.x),
                y: towards_0_1(b1.pos.y)
            };
            flocking + keep_vel + steer_to_screen
        }).collect();
        for (ref mut b, ref accel) in self.boids.iter_mut().zip(accels.iter()) {
            b.vel.axpy(&dt, accel)
        }
    }
}

fn main() {
    let window = Sdl2Window::new(
        OpenGL::_3_2, 
        WindowSettings::new("Boids".to_string(), Size {width: 640, height: 640})
    );
    let mut gl = GlGraphics::new(OpenGL::_3_2);
    let mut app = App::new();
    for e in window.events() {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |context, g| {app.render(context, g);});
        };
        if let Some(args) = e.update_args() {
            app.update_pos(args.dt);
            app.update_vel(args.dt);
        };
    }
}
