extern crate piston_window;
extern crate rand;

mod drawing;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::drawing::to_gui_coord_u32;
use crate::game::Game;

const BACKGROUND_COLOR: Color = [0.08, 0.10, 0.12, 1.0];

fn main() {
    let (width, height) = (27, 22);

    let mut window_settings = WindowSettings::new(
        "🐍 Rust Snake Game",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)],
    )
    .exit_on_esc(true);

    window_settings.set_vsync(true);

    let mut window: PistonWindow = window_settings.build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
