mod asset_loader;
mod input;
mod renderer;
mod sprite_sheet;

use crate::browser;

use anyhow::Result;
use async_trait::async_trait;
use std::{cell::RefCell, rc::Rc};

pub use asset_loader::{ImageAssetLoader, JsonAssetLoader};
pub use input::{
    KeyState, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT, KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
};
pub use renderer::Renderer;
pub use sprite_sheet::{Sheet, SpriteSheet};

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, key_state: &KeyState) -> Result<()>;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;
impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut game = game.initialize().await?;
        let mut game_loop = Self {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };
        let renderer = Renderer {
            context: browser::context()?,
        };
        let mut key_state = KeyState::new();
        let mut keyevent_receiver = input::prepare_input()?;

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);
        *g.borrow_mut() = Some(browser::create_request_animation_frame_closure(
            move |perf| {
                input::process_input(&mut key_state, &mut keyevent_receiver);
                game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
                while game_loop.accumulated_delta > FRAME_SIZE {
                    game.update(&key_state).expect("error GameLoop update");
                    game_loop.accumulated_delta -= FRAME_SIZE;
                }
                game_loop.last_frame = perf;
                game.draw(&renderer).expect("error GameLoop draw");
                browser::request_animation_frame(
                    f.borrow()
                        .as_ref()
                        .expect("error borrowing f ShareLoopClosure"),
                )
                .expect("error request animation frame");
            },
        ));
        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .expect("error borrowing g ShareLoopClosure"),
        )?;
        Ok(())
    }
}

pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
