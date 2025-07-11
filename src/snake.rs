use std::collections::LinkedList;

use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use rand::Rng;

use crate::drawing::{draw_block, draw_eyes, draw_tongue};

const SNAKE_COLOR: Color = [0.30, 0.80, 0.30, 1.0];
const SNAKE_HEAD_COLOR: Color = [0.20, 0.70, 0.20, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    moving_direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>,
    tongue_timer: f64,
    tongue_out: bool,
}

impl Snake {
    pub fn new(init_x: i32, init_y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: init_x + 2,
            y: init_y,
        });
        body.push_back(Block {
            x: init_x + 1,
            y: init_y,
        });
        body.push_back(Block {
            x: init_x,
            y: init_y,
        });

        Snake {
            moving_direction: Direction::Right,
            body,
            last_removed_block: None,
            tongue_timer: 0.0,
            tongue_out: false,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut is_head = true;
        for block in &self.body {
            if is_head {
                draw_block(SNAKE_HEAD_COLOR, block.x, block.y, con, g);
                draw_eyes(self.moving_direction, block.x, block.y, con, g);

                if self.tongue_out {
                    draw_tongue(self.moving_direction, block.x, block.y, con, g);
                }
                is_head = false;
            } else {
                draw_block(SNAKE_COLOR, block.x, block.y, con, g);
            }
        }
    }

    pub fn update_tongue(&mut self, delta_time: f64) {
        self.tongue_timer -= delta_time;
        if self.tongue_timer <= 0.0 {
            self.tongue_out = !self.tongue_out;
            let mut rng = rand::rng();
            self.tongue_timer = if self.tongue_out {
                rng.random_range(0.15..0.25)
            } else {
                rng.random_range(1.0..3.0)
            };
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.moving_direction = d,
            None => {}
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.moving_direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_blk = self.body.pop_back().unwrap();
        self.last_removed_block = Some(removed_blk);
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.moving_direction
    }

    pub fn next_head_position(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.moving_direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_last_removed(&mut self) {
        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn is_overlap_except_tail(&self, x: i32, y: i32) -> bool {
        let mut checked = 0;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            checked += 1;
            if checked == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}
