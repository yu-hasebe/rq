mod actor;

use crate::engine;
use actor::{Actor, ActorStateContext, ActorStateMachine};
use engine::{Game, ImageAssetLoader, JsonAssetLoader, SpriteSheet};

use anyhow::Result;
use async_trait::async_trait;

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
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        let mut json_asset_loader = JsonAssetLoader::new();
        let mut image_asset_loader = ImageAssetLoader::new();
        let sheet = json_asset_loader.load_sheet("Sprite-0001.json").await?;
        let image = image_asset_loader.load_image("Sprite-0001.png").await?;
        Ok(Box::new(Self {
            frame: self.frame,
            actor: Actor {
                state_machine: Some(ActorStateMachine::new(ActorStateContext::new(
                    SpriteSheet {
                        sheet: Some(sheet),
                        image: Some(image),
                    },
                ))),
            },
        }))
    }
    fn update(&mut self, key_state: &engine::KeyState) -> Result<()> {
        if let Some(state_machine) = self.actor.state_machine.take() {
            self.actor
                .state_machine
                .replace(state_machine.update(key_state));
        }
        Ok(())
    }
    fn draw(&self, renderer: &engine::Renderer) -> Result<()> {
        renderer.clear(&engine::Rect {
            x: 0,
            y: 0,
            w: 480,
            h: 480,
        });
        if let Some(state_machine) = &self.actor.state_machine {
            state_machine.draw(renderer)?;
        }
        Ok(())
    }
}
