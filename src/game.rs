mod player;

use crate::browser;
use crate::engine::{Game, KeyState, Rect, Renderer, Sheet, SpriteSheet, SpriteSheetStore};
use player::{Player, PlayerStateContext, PlayerStateMachine};

use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};

const CANVAS_WIDTH: i16 = 480;
const CANVAS_HEIGHT: i16 = 480;
const TILE_WIDTH: i16 = 32;
const TILE_HEIGHT: i16 = 32;

pub struct RQ {
    sprite_sheet_store: SpriteSheetStore,
    player: Player,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            sprite_sheet_store: SpriteSheetStore::new(),
            player: Player::new(),
        }
    }
}

#[async_trait(?Send)]
impl Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let sprite_sheet_store = load_sprite_sheet_store()?;
        let player_state_context = PlayerStateContext::new("Sprite-0001");
        Ok(Box::new(Self {
            sprite_sheet_store,
            player: Player {
                state_machine: Some(PlayerStateMachine::new(player_state_context)),
            },
        }))
    }
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        if let Some(state_machine) = self.player.state_machine.take() {
            self.player
                .state_machine
                .replace(state_machine.update(key_state));
        }
        Ok(())
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            w: CANVAS_WIDTH,
            h: CANVAS_HEIGHT,
        });
        if let Some(state_machine) = &self.player.state_machine {
            state_machine.draw(renderer, &self.sprite_sheet_store)?;
        }
        Ok(())
    }
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
