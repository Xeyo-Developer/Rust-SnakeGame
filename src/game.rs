use piston_window::types::Color;
use piston_window::*;

use rand::Rng;

use crate::drawing::{draw_rectangle, to_gui_coord, BLOCK_SIZE};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.35, 0.35, 0.35, 1.0];
const FOOD_COLOR: Color = [0.96, 0.71, 0.20, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.8];
const SCORE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

const MOVING_PERIOD: f64 = 0.2;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    is_game_over: bool,
    waiting_time: f64,
    food_timer: f64,
    score: u32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 5,
            food_y: 3,
            width,
            height,
            is_game_over: false,
            food_timer: 0.0,
            score: 0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            if key == Key::Space {
                self.restart();
            }
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => return,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.snake.draw(con, g);

        if self.food_exists {
            let alpha = 0.7 + 0.3 * (self.food_timer * 6.0).sin().abs();
            let scale = 1.0 + 0.1 * (self.food_timer * 4.0).sin().abs();
            let mut food_color = FOOD_COLOR;
            food_color[3] = alpha as f32;

            let gui_x = to_gui_coord(self.food_x);
            let gui_y = to_gui_coord(self.food_y);
            let size = BLOCK_SIZE * scale;
            let offset = (BLOCK_SIZE - size) / 2.0;

            rectangle(
                food_color,
                [gui_x + offset, gui_y + offset, size, size],
                con.transform,
                g,
            );
        }

        for i in 0..3 {
            let alpha = 1.0 - (i as f32 * 0.2);
            let border_color = [BORDER_COLOR[0], BORDER_COLOR[1], BORDER_COLOR[2], alpha];
            draw_rectangle(border_color, i, i, self.width - 2 * i, 1, con, g);
            draw_rectangle(
                border_color,
                i,
                self.height - 1 - i,
                self.width - 2 * i,
                1,
                con,
                g,
            );
            draw_rectangle(border_color, i, i, 1, self.height - 2 * i, con, g);
            draw_rectangle(
                border_color,
                self.width - 1 - i,
                i,
                1,
                self.height - 2 * i,
                con,
                g,
            );
        }

        let score_text = format!("Score: {}", self.score);
        let font_size = 20;
        text::Text::new_color(SCORE_COLOR, font_size)
            .draw(
                &score_text,
                glyphs,
                &con.draw_state,
                con.transform.trans(10.0, 30.0),
                g,
            )
            .unwrap();

        if self.is_game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);

            let game_over_text = "GAME OVER";
            let final_score_text = format!("Final Score: {}", self.score);
            let restart_text = "Press SPACE to restart";

            let center_x = to_gui_coord(self.width) / 2.0;
            let center_y = to_gui_coord(self.height) / 2.0;

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                .draw(
                    game_over_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(center_x - 80.0, center_y - 40.0),
                    g,
                )
                .unwrap();

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                .draw(
                    &final_score_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(center_x - 70.0, center_y),
                    g,
                )
                .unwrap();

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                .draw(
                    restart_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(center_x - 80.0, center_y + 40.0),
                    g,
                )
                .unwrap();
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.food_timer += delta_time;
        self.snake.update_tongue(delta_time);

        if self.is_game_over {
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
            self.waiting_time = 0.0;
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_last_removed();
            self.score += 10;
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head_position(dir);

        if self.snake.is_overlap_except_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = rand::rng();

        let mut new_x = rng.random_range(1..(self.width - 1));
        let mut new_y = rng.random_range(1..(self.height - 1));
        while self.snake.is_overlap_except_tail(new_x, new_y) {
            new_x = rng.random_range(1..(self.width - 1));
            new_y = rng.random_range(1..(self.height - 1));
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
        self.food_timer = 0.0;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 5;
        self.food_y = 3;
        self.is_game_over = false;
        self.score = 0;
    }
}
