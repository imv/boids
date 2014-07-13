extern crate piston;
extern crate sdl2_game_window;

use piston::{Game, GameWindowSettings, GameIteratorSettings};
use sdl2_game_window::GameWindowSDL2;

pub struct App;

impl Game for App {
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Boids".to_string(),
            ..GameWindowSettings::default()
        }
    );
    let mut app = App;
    app.run(&mut window,
        &GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );
}
