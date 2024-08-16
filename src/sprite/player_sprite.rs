use crate::{engine::Rect, object::player::Player};

use super::Sprite;

pub struct PlayerSprite {}

impl Sprite for PlayerSprite {
    fn source_image(&self) -> String {
        "Sprite-0001".to_string()
    }
    fn frame(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 32,
            h: 32,
        }
    }
    fn destination(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 32,
            h: 32,
        }
    }
}

impl From<Player> for PlayerSprite {
    fn from(from: Player) -> Self {
        Self {}
    }
}
