mod actor;

use crate::engine::{
    Game, ImageAssetLoader, JsonAssetLoader, KeyState, Rect, Renderer, SpriteSheet,
};
use actor::{Actor, ActorStateContext, ActorStateMachine};

use anyhow::Result;
use async_trait::async_trait;

const CANVAS_WIDTH: i16 = 480;
const CANVAS_HEIGHT: i16 = 480;
const TILE_WIDTH: i16 = 32;
const TILE_HEIGHT: i16 = 32;

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
impl Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let mut json_asset_loader = JsonAssetLoader::new();
        let mut image_asset_loader = ImageAssetLoader::new();
        let sheet = json_asset_loader.load_sheet("Sprite-0001.json").await?;
        let image = image_asset_loader.load_image("Sprite-0001.png").await?;
        let actor_state_context = ActorStateContext::new(SpriteSheet {
            sheet: Some(sheet),
            image: Some(image),
        });
        Ok(Box::new(Self {
            frame: self.frame,
            actor: Actor {
                state_machine: Some(ActorStateMachine::new(actor_state_context)),
            },
        }))
    }
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        if let Some(state_machine) = self.actor.state_machine.take() {
            self.actor
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
        if let Some(state_machine) = &self.actor.state_machine {
            state_machine.draw(renderer)?;
        }
        Ok(())
    }
}
