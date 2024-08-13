use super::{TILE_HEIGHT, TILE_WIDTH};
use crate::engine::{
    KeyState, Point, Renderer, SpriteSheet, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT,
    KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
};

use anyhow::{anyhow, Result};
use std::marker::PhantomData;

pub struct ActorStateContext {
    sprite_sheet: SpriteSheet,
    frame: u8,
    position: Point,
    direction: Direction,
}
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl ActorStateContext {
    pub fn new(sprite_sheet: SpriteSheet) -> Self {
        Self {
            sprite_sheet,
            frame: 0,
            position: Point { x: 0, y: 0 },
            direction: Direction::Down,
        }
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        let frame_name = self.frame_name()?;
        renderer.draw_image(&self.sprite_sheet, &frame_name, &self.position)
    }
    fn move_(&mut self) {
        match self.direction {
            Direction::Left => self.position.x -= 4,
            Direction::Up => self.position.y -= 4,
            Direction::Down => self.position.y += 4,
            Direction::Right => self.position.x += 4,
        }
    }
    fn set_direction(&mut self, direction: Direction) {
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

pub struct Actor {
    pub state_machine: Option<ActorStateMachine>,
}
impl Actor {
    pub fn new() -> Self {
        Self {
            state_machine: None,
        }
    }
}

pub enum ActorStateMachine {
    Idle(ActorState<Idle>),
    Moving(ActorState<Moving>),
}
impl ActorStateMachine {
    pub fn new(context: ActorStateContext) -> Self {
        ActorStateMachine::Idle(ActorState::<Idle>::new(context))
    }
    pub fn update(self, key_state: &KeyState) -> Self {
        match self {
            ActorStateMachine::Idle(state) => state.update(key_state).into(),
            ActorStateMachine::Moving(state) => state.update().into(),
        }
    }
    pub fn draw(&self, renderer: &Renderer) -> Result<()> {
        match self {
            ActorStateMachine::Idle(state) => state.draw(renderer),
            ActorStateMachine::Moving(state) => state.draw(renderer),
        }
    }
}

pub struct Idle;
pub struct Moving;
pub struct ActorState<S> {
    context: ActorStateContext,
    _state: PhantomData<S>,
}
impl<S> ActorState<S> {
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        self.context.draw(renderer)
    }
}

impl ActorState<Idle> {
    fn new(context: ActorStateContext) -> Self {
        Self {
            context,
            _state: PhantomData::<Idle>,
        }
    }
    fn update(mut self, key_state: &KeyState) -> ActorIdleEndState {
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
            return ActorIdleEndState::Complete(ActorState::<Moving> {
                context: self.context,
                _state: PhantomData::<Moving>,
            });
        }

        if key_state.is_pressed(KEY_CODE_ARROW_LEFT) {
            self.context.set_direction(Direction::Left)
        } else if key_state.is_pressed(KEY_CODE_ARROW_UP) {
            self.context.set_direction(Direction::Up);
        } else if key_state.is_pressed(KEY_CODE_ARROW_RIGHT) {
            self.context.set_direction(Direction::Right);
        } else if key_state.is_pressed(KEY_CODE_ARROW_DOWN) {
            self.context.set_direction(Direction::Down);
        }
        ActorIdleEndState::Continue(self)
    }
}

enum ActorIdleEndState {
    Continue(ActorState<Idle>),
    Complete(ActorState<Moving>),
}

impl ActorState<Moving> {
    fn update(mut self) -> ActorMovingEndState {
        self.context.increment_frame();
        self.context.move_();
        if self.context.fit() {
            return ActorMovingEndState::Complete(ActorState::<Idle> {
                context: self.context,
                _state: PhantomData::<Idle>,
            });
        }
        ActorMovingEndState::Continue(self)
    }
}

enum ActorMovingEndState {
    Continue(ActorState<Moving>),
    Complete(ActorState<Idle>),
}

impl From<ActorIdleEndState> for ActorStateMachine {
    fn from(state: ActorIdleEndState) -> Self {
        match state {
            ActorIdleEndState::Complete(state) => ActorStateMachine::Moving(state),
            ActorIdleEndState::Continue(state) => ActorStateMachine::Idle(state),
        }
    }
}
impl From<ActorMovingEndState> for ActorStateMachine {
    fn from(state: ActorMovingEndState) -> Self {
        match state {
            ActorMovingEndState::Complete(state) => ActorStateMachine::Idle(state),
            ActorMovingEndState::Continue(state) => ActorStateMachine::Moving(state),
        }
    }
}
