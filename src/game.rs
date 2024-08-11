use crate::browser;
use crate::engine;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use web_sys::HtmlImageElement;

pub struct RQ {
    frame: u8,
    actor: Actor,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            frame: 0,
            actor: Actor::new(),
        }
    }
}

#[async_trait(?Send)]
impl engine::Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        let json = browser::fetch_json("Sprite-0001.json").await?;
        let sheet: Sheet = serde_wasm_bindgen::from_value(json)
            .map_err(|err| anyhow!("error deserializing json: {:#?}", err))?;
        let image = engine::load_image("Sprite-0001.png").await?;
        Ok(Box::new(Self {
            frame: self.frame,
            actor: Actor {
                state_machine: Some(ActorStateMachine::new(ActorStateContext::new(sheet, image))),
            },
        }))
    }
    fn update(&mut self, key_state: &engine::KeyState) {
        if let Some(state_machine) = self.actor.state_machine.take() {
            self.actor
                .state_machine
                .replace(state_machine.update(key_state));
        }
    }
    fn draw(&self, renderer: &engine::Renderer) {
        renderer.clear(&engine::Rect {
            x: 0,
            y: 0,
            w: 480,
            h: 480,
        });
        if let Some(state_machine) = &self.actor.state_machine {
            state_machine.draw(renderer);
        }
    }
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

struct Actor {
    state_machine: Option<ActorStateMachine>,
}
impl Actor {
    fn new() -> Self {
        Self {
            state_machine: None,
        }
    }
}
enum ActorStateMachine {
    Idle(ActorState<Idle>),
    Moving(ActorState<Moving>),
}
impl ActorStateMachine {
    fn new(context: ActorStateContext) -> Self {
        ActorStateMachine::Idle(ActorState::<Idle>::new(context))
    }
    fn update(self, key_state: &engine::KeyState) -> Self {
        match self {
            ActorStateMachine::Idle(state) => {
                let state = state.update(key_state);
                match state {
                    ActorIdleEndState::Complete(state) => ActorStateMachine::Moving(state),
                    ActorIdleEndState::Continue(state) => ActorStateMachine::Idle(state),
                }
            }
            ActorStateMachine::Moving(state) => {
                let state = state.update();
                match state {
                    ActorMovingEndState::Complete(state) => ActorStateMachine::Idle(state),
                    ActorMovingEndState::Continue(state) => ActorStateMachine::Moving(state),
                }
            }
        }
    }
    fn draw(&self, renderer: &engine::Renderer) {
        match self {
            ActorStateMachine::Idle(state) => state.draw(renderer),
            ActorStateMachine::Moving(state) => state.draw(renderer),
        }
    }
}

struct Idle;
struct Moving;
struct ActorState<S> {
    context: ActorStateContext,
    _state: PhantomData<S>,
}
impl<S> ActorState<S> {
    fn draw(&self, renderer: &engine::Renderer) {
        let sprite_name = match self.context.direction {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Right => "right",
            Direction::Left => "left",
        };
        let frame: u8 = match self.context.frame / 6 {
            0 | 2 => 2,
            1 => 3,
            3 => 1,
            _ => panic!(),
        };
        let frame_name = format!("{}0{}.png", sprite_name, frame);
        let sprite = self
            .context
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .unwrap();
        self.context
            .image
            .as_ref()
            .map(|image| {
                renderer.draw_image(
                    image,
                    &engine::Rect {
                        x: sprite.frame.x.into(),
                        y: sprite.frame.y.into(),
                        w: sprite.frame.w.into(),
                        h: sprite.frame.h.into(),
                    },
                    &engine::Rect {
                        x: self.context.position.x.into(),
                        y: self.context.position.y.into(),
                        w: sprite.frame.w.into(),
                        h: sprite.frame.h.into(),
                    },
                )
            })
            .unwrap();
    }
}
struct ActorStateContext {
    frame: u8,
    sheet: Option<Sheet>,
    image: Option<HtmlImageElement>,
    position: engine::Point,
    direction: Direction,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl ActorStateContext {
    fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        Self {
            frame: 0,
            sheet: Some(sheet),
            image: Some(image),
            position: engine::Point { x: 0, y: 0 },
            direction: Direction::Down,
        }
    }
}

impl ActorState<Idle> {
    fn new(context: ActorStateContext) -> Self {
        Self {
            context,
            _state: PhantomData::<Idle>,
        }
    }
    fn update(mut self, key_state: &engine::KeyState) -> ActorIdleEndState {
        log!("x: {}, y: {}", self.context.position.x, self.context.position.y);
        if key_state.is_pressed("ArrowUp") {
            if self.context.direction == Direction::Up {
                self.context.position.y -= 4;
                return ActorIdleEndState::Complete(ActorState::<Moving> {
                    context: self.context,
                    _state: PhantomData::<Moving>,
                });
            }
            self.context.direction = Direction::Up;
        } else if key_state.is_pressed("ArrowDown") {
            if self.context.direction == Direction::Down {
                self.context.position.y += 4;
                return ActorIdleEndState::Complete(ActorState::<Moving> {
                    context: self.context,
                    _state: PhantomData::<Moving>,
                });
            }
            self.context.direction = Direction::Down;
        } else if key_state.is_pressed("ArrowRight") {
            if self.context.direction == Direction::Right {
                self.context.position.x += 4;
                return ActorIdleEndState::Complete(ActorState::<Moving> {
                    context: self.context,
                    _state: PhantomData::<Moving>,
                });
            }
            self.context.direction = Direction::Right;
        } else if key_state.is_pressed("ArrowLeft") {
            if self.context.direction == Direction::Left {
                self.context.position.x -= 4;
                return ActorIdleEndState::Complete(ActorState::<Moving> {
                    context: self.context,
                    _state: PhantomData::<Moving>,
                });
            }
            self.context.direction = Direction::Left;
        }
        return ActorIdleEndState::Continue(self);
    }
}

enum ActorIdleEndState {
    Continue(ActorState<Idle>),
    Complete(ActorState<Moving>),
}

impl ActorState<Moving> {
    fn update(mut self) -> ActorMovingEndState {
        log!("x: {}, y: {}", self.context.position.x, self.context.position.y);
        match self.context.direction {
            Direction::Up => self.context.position.y -= 4,
            Direction::Down => self.context.position.y += 4,
            Direction::Right => self.context.position.x += 4,
            Direction::Left => self.context.position.x -= 4,
        };
        if self.context.position.x % 32 == 0 && self.context.position.y % 32 == 0 {
            ActorMovingEndState::Complete(ActorState::<Idle> {
                context: self.context,
                _state: PhantomData::<Idle>,
            })
        } else {
            ActorMovingEndState::Continue(self)
        }
    }
}

enum ActorMovingEndState {
    Continue(ActorState<Moving>),
    Complete(ActorState<Idle>),
}
