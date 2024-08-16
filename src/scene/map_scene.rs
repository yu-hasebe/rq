use crate::engine::Renderer;
use crate::{engine::KeyState, object::player::Player};
use crate::sprite::player_sprite::{FooValue, PlayerSprite};

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
        let foo = FooValue{value: player};
        let r: &Player = &foo;
        let sprite: PlayerSprite = r.into();
        renderer.render(sprite)
    }
}
