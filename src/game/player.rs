use crate::engine::{
    GameObject, KeyState, Point, Renderer, SpriteSheet, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT,
    KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::{TILE_HEIGHT, TILE_WIDTH};

#[derive(Deserialize, Serialize)]
pub struct Player {
    state: PlayerState,
    source: String,
    position: Point,
    direction: Direction,
    #[serde(skip)]
    frame: u8,
    #[serde(skip)]
    sprite_sheet: Option<SpriteSheet>, // FIX ME
}

#[derive(Deserialize, Serialize)]
pub enum PlayerState {
    Stopped,
    Moving,
}

#[derive(Deserialize, Serialize)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl GameObject<'_> for Player {
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        match self.state {
            PlayerState::Stopped => {
                self.reset_frame();
                match self.direction {
                    Direction::Left => {
                        if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
                            self.move_();
                        }
                    }
                    Direction::Up => {
                        if key_state.is_pressed(KEY_CODE_ARROW_UP) {
                            self.move_();
                        }
                    }
                    Direction::Right => {
                        if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
                            self.move_();
                        }
                    }
                    Direction::Down => {
                        if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
                            self.move_();
                        }
                    }
                }
                if !self.fit() {
                    self.increment_frame();
                    self.state = PlayerState::Moving;
                    return Ok(());
                }

                if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
                    self.change_direction(Direction::Left);
                } else if key_state.is_pressed(KEY_CODE_ARROW_UP) {
                    self.change_direction(Direction::Up);
                } else if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
                    self.change_direction(Direction::Right);
                } else if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
                    self.change_direction(Direction::Down);
                }
                Ok(())
            }
            PlayerState::Moving => {
                self.increment_frame();
                self.move_();
                if self.fit() {
                    self.state = PlayerState::Stopped;
                    return Ok(());
                }
                Ok(())
            }
        }
    }

    fn draw(&self, renderer: &Renderer) -> Result<()> {
        let frame_name = self.frame_name()?;
        let sprite_sheet = self
            .sprite_sheet
            .as_ref()
            .ok_or_else(|| anyhow!("no SpriteSheet found"))?;
        renderer.draw_image(sprite_sheet, &frame_name, &self.position)
    }
}

impl Player {
    pub fn new(source: &str, sprite_sheet: Option<SpriteSheet>) -> Self {
        Self {
            state: PlayerState::Stopped,
            source: source.to_string(),
            position: Point { x: 0, y: 0 },
            direction: Direction::Down,
            frame: 0,
            sprite_sheet,
        }
    }
    fn move_(&mut self) {
        match self.direction {
            Direction::Left => self.position.x -= 4,
            Direction::Up => self.position.y -= 4,
            Direction::Down => self.position.y += 4,
            Direction::Right => self.position.x += 4,
        }
    }
    fn fit(&self) -> bool {
        self.position.x % TILE_WIDTH == 0 && self.position.y % TILE_HEIGHT == 0
    }
    fn reset_frame(&mut self) {
        self.frame = 0;
    }
    fn increment_frame(&mut self) {
        self.frame = (self.frame + 1) % 16;
    }
    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    fn frame_name(&self) -> Result<String> {
        let frame_name = match self.direction {
            Direction::Left => "left",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Right => "right",
        };
        let frame: u8 = match self.frame / 4 {
            0 | 2 => 2,
            1 => 3,
            3 => 1,
            _ => return Err(anyhow!("invalid frame logic")),
        };
        Ok(format!("{}0{}.png", frame_name, frame))
    }
}
