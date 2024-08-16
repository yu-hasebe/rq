use crate::engine::Renderer;
use crate::{engine::KeyState, object::player::Player};
use crate::sprite::player_sprite::PlayerSprite;

use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

use super::Scene;

#[derive(Clone)]
pub struct MapScene {
    pub player: Rc<RefCell<Player>>,
}

impl Scene for MapScene {
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        Ok(())
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        let player = self.player.borrow();
        let sprite = PlayerSprite{};
        renderer.render(sprite)
    }
}
