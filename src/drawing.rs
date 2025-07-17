use piston_window::types::Color;
use piston_window::*;

use crate::snake::Direction;

pub const BLOCK_SIZE: f64 = 25.0;

pub fn to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(game_coord: i32) -> u32 {
    to_gui_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(
        [color[0] * 0.7, color[1] * 0.7, color[2] * 0.7, color[3]],
        [gui_x + 2.0, gui_y + 2.0, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    start_x: i32,
    start_y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);

    rectangle(
        color,
        [
            gui_start_x,
            gui_start_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_eyes(direction: Direction, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);
    let eye_color = [1.0, 1.0, 1.0, 1.0];
    let pupil_color = [0.1, 0.1, 0.1, 1.0];

    let (offset_x1, offset_y1, offset_x2, offset_y2) = match direction {
        Direction::Up => (0.25, 0.25, 0.75, 0.25),
        Direction::Down => (0.25, 0.75, 0.75, 0.75),
        Direction::Left => (0.25, 0.25, 0.25, 0.75),
        Direction::Right => (0.75, 0.25, 0.75, 0.75),
    };

    for &(offset_x, offset_y) in &[(offset_x1, offset_y1), (offset_x2, offset_y2)] {
        ellipse(
            eye_color,
            [
                gui_x + offset_x * BLOCK_SIZE - 4.0,
                gui_y + offset_y * BLOCK_SIZE - 4.0,
                8.0,
                8.0,
            ],
            con.transform,
            g,
        );

        ellipse(
            pupil_color,
            [
                gui_x + offset_x * BLOCK_SIZE - 2.0,
                gui_y + offset_y * BLOCK_SIZE - 2.0,
                4.0,
                4.0,
            ],
            con.transform,
            g,
        );
    }
}

pub fn draw_tongue(direction: Direction, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);
    let tongue_color = [0.86, 0.22, 0.27, 1.0];

    let (start_x, start_y, end_x, end_y) = match direction {
        Direction::Up => (0.5, 0.0, 0.5, -0.3),
        Direction::Down => (0.5, 1.0, 0.5, 1.3),
        Direction::Left => (0.0, 0.5, -0.3, 0.5),
        Direction::Right => (1.0, 0.5, 1.3, 0.5),
    };

    rectangle(
        tongue_color,
        [
            gui_x + start_x * BLOCK_SIZE - 2.0,
            gui_y + start_y * BLOCK_SIZE - 2.0,
            4.0,
            BLOCK_SIZE * 0.3,
        ],
        con.transform,
        g,
    );

    ellipse(
        tongue_color,
        [
            gui_x + end_x * BLOCK_SIZE - 3.0,
            gui_y + end_y * BLOCK_SIZE - 3.0,
            6.0,
            6.0,
        ],
        con.transform,
        g,
    );
}
