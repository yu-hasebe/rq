use crate::engine::Renderer;
use crate::sprite::player_sprite::PlayerSprite;
use crate::sprite::ConverterIntoSprite;
use crate::{engine::KeyState, object::player::Player};

use anyhow::Result;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use super::Scene;

#[derive(Clone)]
pub struct MapScene {
    pub player: Rc<RefCell<Player>>,
}

impl Scene for MapScene {
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        let mut player = self.player.borrow_mut();
        player.update(key_state)?;
        Ok(())
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        let sprite: PlayerSprite = ConverterIntoSprite {
            value: self.player.borrow(),
        }
        .deref()
        .into();
        renderer.render(sprite)
    }
}
