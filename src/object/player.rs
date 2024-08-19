use crate::{
    engine::{
        KeyState, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT, KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
    },
    scene::{TILE_HEIGHT, TILE_WIDTH},
};

use anyhow::Result;

pub struct Player {
    pub frame: u8,
    pub direction: Direction,

    pub x: i16,
    pub y: i16,
    pub map_id: usize,

    pub next_x: Option<i16>,
    pub next_y: Option<i16>,
    pub next_map_id: Option<usize>,

    pub pixel_x: i16,
    pub pixel_y: i16,
    pub move_speed: i16,
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Player {
    pub fn new(x: i16, y: i16, map_id: usize, move_speed: i16) -> Self {
        Self {
            frame: 0,
            direction: Direction::Down,

            x,
            y,
            map_id,

            next_x: None,
            next_y: None,
            next_map_id: None,

            pixel_x: 0,
            pixel_y: 0,
            move_speed,
        }
    }
}

impl Player {
    pub fn update(&mut self, key_state: &KeyState) -> Result<()> {
        if self.is_moving() {
            self.increment_frame();
            match self.direction {
                Direction::Left => {
                    self.pixel_x -= self.move_speed;
                },
                Direction::Up => {
                    self.pixel_y -= self.move_speed;
                },
                Direction::Right => {
                    self.pixel_x += self.move_speed;
                },
                Direction::Down => {
                    self.pixel_y += self.move_speed;
                },
            }
            return Ok(());
        }
        self.reset_frame();

        match self.direction {
            Direction::Left => {
                if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
                    self.pixel_x -= self.move_speed;
                }
            }
            Direction::Up => {
                if key_state.is_pressed(KEY_CODE_ARROW_UP) {
                    self.pixel_y -= self.move_speed;
                }
            }
            Direction::Right => {
                if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
                    self.pixel_x += self.move_speed;
                }
            }
            Direction::Down => {
                if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
                    self.pixel_y += self.move_speed;
                }
            }
        }

        if self.is_moving() {
            self.increment_frame();
            return Ok(());
        }

        if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
            self.direction = Direction::Left;
        } else if key_state.is_pressed(KEY_CODE_ARROW_UP) {
            self.direction = Direction::Up;
        } else if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
            self.direction = Direction::Right;
        } else if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
            self.direction = Direction::Down;
        }

        Ok(())
    }
}

impl Player {
    fn reset_frame(&mut self) {
        self.frame = 0;
    }
    fn increment_frame(&mut self) {
        self.frame = (self.frame + 1) % 16;
    }
    fn is_moving(&self) -> bool {
        self.pixel_x % TILE_WIDTH > 0 && self.pixel_y % TILE_HEIGHT > 0
    }
}