use super::{TILE_HEIGHT, TILE_WIDTH};
use crate::engine::{
    KeyState, Point, Renderer, SpriteSheetStore, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT,
    KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct Player {
    pub state_machine: Option<PlayerStateMachine>,
}
impl Player {
    pub fn new() -> Self {
        Self {
            state_machine: None,
        }
    }
}

pub enum PlayerStateMachine {
    Stopped(PlayerState<Stopped>),
    Moving(PlayerState<Moving>),
}
impl PlayerStateMachine {
    pub fn new(context: PlayerStateContext) -> Self {
        PlayerStateMachine::Stopped(PlayerState::<Stopped>::new(context))
    }
    pub fn update(self, key_state: &KeyState) -> Self {
        match self {
            PlayerStateMachine::Stopped(state) => state.update(key_state).into(),
            PlayerStateMachine::Moving(state) => state.update().into(),
        }
    }
    pub fn draw(&self, renderer: &Renderer, sprite_sheet_store: &SpriteSheetStore) -> Result<()> {
        match self {
            PlayerStateMachine::Stopped(state) => state.draw(renderer, sprite_sheet_store),
            PlayerStateMachine::Moving(state) => state.draw(renderer, sprite_sheet_store),
        }
    }
}

pub struct Stopped;
pub struct Moving;
#[derive(Deserialize, Serialize)]
pub struct PlayerState<S> {
    #[serde(flatten)]
    context: PlayerStateContext,
    state: PhantomData<S>,
}
impl<S> PlayerState<S> {
    fn draw(&self, renderer: &Renderer, sprite_sheet_store: &SpriteSheetStore) -> Result<()> {
        let frame_name = self.context.frame_name()?;
        let sprite_sheet = sprite_sheet_store.get(&self.context.sprite_source)?;
        renderer.draw_image(sprite_sheet, &frame_name, &self.context.position)
    }
}

impl PlayerState<Stopped> {
    fn new(context: PlayerStateContext) -> Self {
        Self {
            context,
            state: PhantomData::<Stopped>,
        }
    }
    fn update(mut self, key_state: &KeyState) -> PlayerStoppedEndState {
        self.context.reset_frame();
        match self.context.direction {
            Direction::Left => {
                if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
                    self.context.move_();
                }
            }
            Direction::Up => {
                if key_state.is_pressed(KEY_CODE_ARROW_UP) {
                    self.context.move_();
                }
            }
            Direction::Right => {
                if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
                    self.context.move_();
                }
            }
            Direction::Down => {
                if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
                    self.context.move_();
                }
            }
        }
        if !self.context.fit() {
            self.context.increment_frame();
            return PlayerStoppedEndState::Complete(PlayerState::<Moving> {
                context: self.context,
                state: PhantomData::<Moving>,
            });
        }

        if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
            self.context.change_direction(Direction::Left)
        } else if key_state.is_pressed(KEY_CODE_ARROW_UP) {
            self.context.change_direction(Direction::Up);
        } else if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
            self.context.change_direction(Direction::Right);
        } else if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
            self.context.change_direction(Direction::Down);
        }
        PlayerStoppedEndState::Continue(self)
    }
}

enum PlayerStoppedEndState {
    Continue(PlayerState<Stopped>),
    Complete(PlayerState<Moving>),
}

impl PlayerState<Moving> {
    fn update(mut self) -> PlayerMovingEndState {
        self.context.increment_frame();
        self.context.move_();
        if self.context.fit() {
            return PlayerMovingEndState::Complete(PlayerState::<Stopped> {
                context: self.context,
                state: PhantomData::<Stopped>,
            });
        }
        PlayerMovingEndState::Continue(self)
    }
}

enum PlayerMovingEndState {
    Continue(PlayerState<Moving>),
    Complete(PlayerState<Stopped>),
}

impl From<PlayerStoppedEndState> for PlayerStateMachine {
    fn from(state: PlayerStoppedEndState) -> Self {
        match state {
            PlayerStoppedEndState::Complete(state) => PlayerStateMachine::Moving(state),
            PlayerStoppedEndState::Continue(state) => PlayerStateMachine::Stopped(state),
        }
    }
}
impl From<PlayerMovingEndState> for PlayerStateMachine {
    fn from(state: PlayerMovingEndState) -> Self {
        match state {
            PlayerMovingEndState::Complete(state) => PlayerStateMachine::Stopped(state),
            PlayerMovingEndState::Continue(state) => PlayerStateMachine::Moving(state),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlayerStateContext {
    sprite_source: String,
    position: Point,
    direction: Direction,
    #[serde(skip)]
    frame: u8,
}
#[derive(Deserialize, Serialize)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl PlayerStateContext {
    pub fn new(sprite_source: &str) -> Self {
        Self {
            sprite_source: sprite_source.to_string(),
            frame: 0,
            position: Point { x: 0, y: 0 },
            direction: Direction::Down,
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
    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
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
