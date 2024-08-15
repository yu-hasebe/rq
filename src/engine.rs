mod input;
mod renderer;
mod sprite_sheet;

use crate::browser;

use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

pub use input::{
    KeyState, KEY_CODE_ARROW_DOWN, KEY_CODE_ARROW_LEFT, KEY_CODE_ARROW_RIGHT, KEY_CODE_ARROW_UP,
};
pub use renderer::Renderer;
pub use sprite_sheet::{Sheet, SpriteSheet, SpriteSheetStore};

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, key_state: &KeyState) -> Result<()>;
    fn draw(&self, renderer: &Renderer, sprite_sheet_store: &SpriteSheetStore) -> Result<()>;
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
        let sprite_sheet_store = load_sprite_sheet_store()?;
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
                game.draw(&renderer, &sprite_sheet_store).expect("error GameLoop draw");
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

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

// TODO I'd like to load files in `asset/*` automatically
fn load_sprite_sheet() -> Result<SpriteSheet> {
    let image = include_bytes!("asset/Sprite-0001.png");
    let json = include_bytes!("asset/Sprite-0001.json");

    let src = format!(
        "data:image/{};base64,{}",
        "png",
        STANDARD.encode(&image.to_vec())
    );
    let image_element = browser::new_image()?;
    image_element.set_src(&src);

    let sheet: Sheet = serde_json::from_slice(json)?;
    Ok(SpriteSheet {
        name: "Sprite-0001".to_string(),
        sheet: Some(sheet),
        image: Some(image_element),
    })
}

fn load_sprite_sheet_store() -> Result<SpriteSheetStore> {
    let sprite_sheet = load_sprite_sheet()?;
    let mut sprite_sheet_store = SpriteSheetStore::new();
    sprite_sheet_store.add(sprite_sheet)?;
    Ok(sprite_sheet_store)
}
