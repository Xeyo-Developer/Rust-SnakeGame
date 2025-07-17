use piston_window::types::Color;
use piston_window::*;

use rand::Rng;

use crate::drawing::{draw_rectangle, to_gui_coord, BLOCK_SIZE};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.35, 0.35, 0.35, 1.0];
const FOOD_COLOR: Color = [0.96, 0.71, 0.20, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.8];
const SCORE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

const SPEED_POWERUP_COLOR: Color = [0.0, 0.8, 1.0, 1.0];
const SHIELD_POWERUP_COLOR: Color = [0.8, 0.0, 1.0, 1.0];
const SHIELD_EFFECT_COLOR: Color = [0.8, 0.0, 1.0, 0.3];

const MOVING_PERIOD: f64 = 0.2;
const SPEED_BOOST_PERIOD: f64 = 0.1;
const SPEED_BOOST_DURATION: f64 = 5.0;
const SHIELD_DURATION: f64 = 10.0;
const POWERUP_SPAWN_CHANCE: f64 = 0.02;

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    Speed,
    Shield,
}

pub struct PowerUp {
    pub x: i32,
    pub y: i32,
    pub powerup_type: PowerUpType,
    pub timer: f64,
}

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

    powerups: Vec<PowerUp>,
    speed_boost_timer: f64,
    shield_timer: f64,
    is_speed_boosted: bool,
    is_shielded: bool,

    screen_shake_timer: f64,
    screen_shake_intensity: f64,

    combo_count: u32,
    combo_timer: f64,

    game_over_animation: f64,

    high_score: u32,
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
            powerups: Vec::new(),
            speed_boost_timer: 0.0,
            shield_timer: 0.0,
            is_speed_boosted: false,
            is_shielded: false,
            screen_shake_timer: 0.0,
            screen_shake_intensity: 0.0,
            combo_count: 0,
            combo_timer: 0.0,
            game_over_animation: 0.0,
            high_score: 0,
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
        let shake_x = if self.screen_shake_timer > 0.0 {
            let mut rng = rand::rng();
            rng.random_range(-self.screen_shake_intensity..self.screen_shake_intensity)
        } else {
            0.0
        };
        let shake_y = if self.screen_shake_timer > 0.0 {
            let mut rng = rand::rng();
            rng.random_range(-self.screen_shake_intensity..self.screen_shake_intensity)
        } else {
            0.0
        };

        let shake_transform = con.transform.trans(shake_x, shake_y);
        let shake_con = con.trans(shake_x, shake_y);

        if self.is_shielded {
            let pulse = 0.7 + 0.3 * (self.shield_timer * 8.0).sin().abs();
            let mut shield_color = SHIELD_EFFECT_COLOR;
            shield_color[3] = pulse as f32 * 0.3;

            draw_rectangle(
                shield_color,
                1,
                1,
                self.width - 2,
                self.height - 2,
                &shake_con,
                g,
            );
        }

        self.snake.draw(&shake_con, g);

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
                shake_transform,
                g,
            );
        }

        for powerup in &self.powerups {
            let color = match powerup.powerup_type {
                PowerUpType::Speed => SPEED_POWERUP_COLOR,
                PowerUpType::Shield => SHIELD_POWERUP_COLOR,
            };

            let pulse = 0.5 + 0.5 * (powerup.timer * 5.0).sin().abs();
            let mut powerup_color = color;
            powerup_color[3] = pulse as f32;

            let gui_x = to_gui_coord(powerup.x);
            let gui_y = to_gui_coord(powerup.y);
            let size = BLOCK_SIZE * 0.8;
            let offset = (BLOCK_SIZE - size) / 2.0;

            rectangle(
                powerup_color,
                [gui_x + offset, gui_y + offset, size, size],
                shake_transform,
                g,
            );
        }

        for i in 0..3 {
            let alpha = 1.0 - (i as f32 * 0.2);
            let border_color = [BORDER_COLOR[0], BORDER_COLOR[1], BORDER_COLOR[2], alpha];
            draw_rectangle(border_color, i, i, self.width - 2 * i, 1, &shake_con, g);
            draw_rectangle(
                border_color,
                i,
                self.height - 1 - i,
                self.width - 2 * i,
                1,
                &shake_con,
                g,
            );
            draw_rectangle(border_color, i, i, 1, self.height - 2 * i, &shake_con, g);
            draw_rectangle(
                border_color,
                self.width - 1 - i,
                i,
                1,
                self.height - 2 * i,
                &shake_con,
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

        if self.high_score > 0 {
            let high_score_text = format!("Best: {}", self.high_score);
            text::Text::new_color([0.8, 0.8, 0.8, 1.0], 16)
                .draw(
                    &high_score_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(10.0, 55.0),
                    g,
                )
                .unwrap();
        }

        let mut status_y = 80.0;
        if self.is_speed_boosted {
            let speed_text = format!("SPEED BOOST: {:.1}s", self.speed_boost_timer);
            text::Text::new_color(SPEED_POWERUP_COLOR, 14)
                .draw(
                    &speed_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(10.0, status_y),
                    g,
                )
                .unwrap();
            status_y += 20.0;
        }

        if self.is_shielded {
            let shield_text = format!("SHIELD: {:.1}s", self.shield_timer);
            text::Text::new_color(SHIELD_POWERUP_COLOR, 14)
                .draw(
                    &shield_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(10.0, status_y),
                    g,
                )
                .unwrap();
            status_y += 20.0;
        }

        if self.combo_count > 1 {
            let combo_text = format!("COMBO x{}", self.combo_count);
            let combo_color = [
                1.0,
                1.0 - (self.combo_count as f32 * 0.1).min(0.5),
                0.0,
                1.0,
            ];
            text::Text::new_color(combo_color, 16)
                .draw(
                    &combo_text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans(10.0, status_y),
                    g,
                )
                .unwrap();
        }

        if self.is_game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, &shake_con, g);

            let center_x = to_gui_coord(self.width) / 2.0;
            let center_y = to_gui_coord(self.height) / 2.0;

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 36)
                .draw(
                    "GAME OVER",
                    glyphs,
                    &con.draw_state,
                    shake_transform.trans(center_x - 90.0, center_y - 40.0),
                    g,
                )
                .unwrap();

            let score_text = format!("Score: {}", self.score);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 28)
                .draw(
                    &score_text,
                    glyphs,
                    &con.draw_state,
                    shake_transform.trans(center_x - 60.0, center_y),
                    g,
                )
                .unwrap();

            if self.high_score > 0 {
                let high_score_text = format!("High Score: {}", self.high_score);
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                    .draw(
                        &high_score_text,
                        glyphs,
                        &con.draw_state,
                        shake_transform.trans(center_x - 80.0, center_y + 40.0),
                        g,
                    )
                    .unwrap();

                if self.score == self.high_score {
                    text::Text::new_color([1.0, 0.84, 0.0, 1.0], 20)
                        .draw(
                            "NEW RECORD!",
                            glyphs,
                            &con.draw_state,
                            shake_transform.trans(center_x - 60.0, center_y + 70.0),
                            g,
                        )
                        .unwrap();
                }
            }

            let blink = (self.game_over_animation * 2.0).sin().abs() as f32;
            text::Text::new_color([1.0, 1.0, 1.0, blink], 20)
                .draw(
                    "Press SPACE to restart",
                    glyphs,
                    &con.draw_state,
                    shake_transform.trans(center_x - 100.0, center_y + 100.0),
                    g,
                )
                .unwrap();
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.food_timer += delta_time;
        self.snake.update_tongue(delta_time);

        if self.speed_boost_timer > 0.0 {
            self.speed_boost_timer -= delta_time;
            if self.speed_boost_timer <= 0.0 {
                self.is_speed_boosted = false;
            }
        }

        if self.shield_timer > 0.0 {
            self.shield_timer -= delta_time;
            if self.shield_timer <= 0.0 {
                self.is_shielded = false;
            }
        }

        if self.screen_shake_timer > 0.0 {
            self.screen_shake_timer -= delta_time;
            self.screen_shake_intensity *= 0.9;
        }

        if self.combo_timer > 0.0 {
            self.combo_timer -= delta_time;
            if self.combo_timer <= 0.0 {
                self.combo_count = 0;
            }
        }

        if self.is_game_over {
            self.game_over_animation += delta_time;
            return;
        }

        self.powerups.retain_mut(|powerup| {
            powerup.timer += delta_time;
            powerup.timer < 10.0
        });

        if rand::rng().random::<f64>() < POWERUP_SPAWN_CHANCE {
            self.spawn_powerup();
        }

        if !self.food_exists {
            self.add_food();
        }

        let movement_period = if self.is_speed_boosted {
            SPEED_BOOST_PERIOD
        } else {
            MOVING_PERIOD
        };

        if self.waiting_time > movement_period {
            self.update_snake(None);
            self.waiting_time = 0.0;
        }
    }

    fn spawn_powerup(&mut self) {
        let mut rng = rand::rng();

        let mut new_x = rng.random_range(1..(self.width - 1));
        let mut new_y = rng.random_range(1..(self.height - 1));

        while self.snake.is_overlap_except_tail(new_x, new_y)
            || (self.food_exists && new_x == self.food_x && new_y == self.food_y)
        {
            new_x = rng.random_range(1..(self.width - 1));
            new_y = rng.random_range(1..(self.height - 1));
        }

        let powerup_type = if rng.random::<f64>() < 0.5 {
            PowerUpType::Speed
        } else {
            PowerUpType::Shield
        };

        self.powerups.push(PowerUp {
            x: new_x,
            y: new_y,
            powerup_type,
            timer: 0.0,
        });
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_last_removed();

            self.combo_count += 1;
            self.combo_timer = 3.0;

            let combo_bonus = if self.combo_count > 1 {
                self.combo_count * 2
            } else {
                0
            };

            self.score += 10 + combo_bonus;

            self.screen_shake_timer = 0.1;
            self.screen_shake_intensity = 2.0;
        }

        let mut powerup_to_remove = None;
        for (i, powerup) in self.powerups.iter().enumerate() {
            if powerup.x == head_x && powerup.y == head_y {
                powerup_to_remove = Some((i, powerup.powerup_type));
                break;
            }
        }

        if let Some((index, powerup_type)) = powerup_to_remove {
            match powerup_type {
                PowerUpType::Speed => {
                    self.is_speed_boosted = true;
                    self.speed_boost_timer = SPEED_BOOST_DURATION;
                }
                PowerUpType::Shield => {
                    self.is_shielded = true;
                    self.shield_timer = SHIELD_DURATION;
                }
            }

            self.score += 50;
            self.screen_shake_timer = 0.2;
            self.screen_shake_intensity = 4.0;
            self.powerups.remove(index);
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head_position(dir);

        if self.snake.is_overlap_except_tail(next_x, next_y) {
            return self.is_shielded;
        }

        let wall_collision =
            next_x <= 0 || next_y <= 0 || next_x >= self.width - 1 || next_y >= self.height - 1;

        if wall_collision {
            return self.is_shielded;
        }

        true
    }

    fn add_food(&mut self) {
        let mut rng = rand::rng();

        let mut new_x = rng.random_range(1..(self.width - 1));
        let mut new_y = rng.random_range(1..(self.height - 1));

        while self.snake.is_overlap_except_tail(new_x, new_y)
            || self.powerups.iter().any(|p| p.x == new_x && p.y == new_y)
        {
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
            self.game_over_animation = 0.0;
            self.speed_boost_timer = 0.0;
            self.shield_timer = 0.0;
            self.is_speed_boosted = false;
            self.is_shielded = false;
            self.screen_shake_timer = 0.0;

            if self.score > self.high_score {
                self.high_score = self.score;
            }
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
        self.powerups.clear();
        self.speed_boost_timer = 0.0;
        self.shield_timer = 0.0;
        self.is_speed_boosted = false;
        self.is_shielded = false;
        self.screen_shake_timer = 0.0;
        self.screen_shake_intensity = 0.0;
        self.combo_count = 0;
        self.combo_timer = 0.0;
        self.game_over_animation = 0.0;
    }
}
