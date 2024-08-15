mod floor;
mod player;

use crate::engine::{Game, KeyState, Rect, Renderer, SpriteSheetStore};
use player::{Player, PlayerStateContext, PlayerStateMachine};

use anyhow::Result;
use async_trait::async_trait;

const CANVAS_WIDTH: i16 = 480;
const CANVAS_HEIGHT: i16 = 480;
const TILE_WIDTH: i16 = 32;
const TILE_HEIGHT: i16 = 32;

pub struct RQ {
    player: Player,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }
}

#[async_trait(?Send)]
impl Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let player_state_context = PlayerStateContext::new("Sprite-0001");
        Ok(Box::new(Self {
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
    fn draw(&self, renderer: &Renderer, sprite_sheet_store: &SpriteSheetStore) -> Result<()> {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            w: CANVAS_WIDTH,
            h: CANVAS_HEIGHT,
        });
        if let Some(state_machine) = &self.player.state_machine {
            state_machine.draw(renderer, sprite_sheet_store)?;
        }
        Ok(())
    }
}
