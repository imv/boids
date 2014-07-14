extern crate piston;
extern crate graphics;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use piston::{Game, GameWindowSettings, GameIteratorSettings, RenderArgs};
use graphics::{Context, AddColor, Draw};
use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;

pub struct App {
    gl: Gl
}

impl App {
    fn new() -> App {
        App {
            gl: Gl::new()
        }
    }
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);
        Context::abs(args.width as f64, args.height as f64)
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
