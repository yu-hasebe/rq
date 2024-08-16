use std::{borrow::Borrow, cell::Ref, ops::Deref};

use crate::{engine::Rect, object::player::{Direction, Player}};

use super::Sprite;

pub struct PlayerSprite {
    direction: Direction,
}

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

pub struct FooValue<'a> {
    pub value: Ref<'a, Player>,
}

impl<'b> Deref for FooValue<'b> {
    type Target = Player;
    fn deref(&self) -> &Player {
        &self.value
    }
}

impl From<&Player> for PlayerSprite {
    fn from(from: &Player) -> PlayerSprite {
        PlayerSprite {
            direction: from.direction.clone(),
        }
    }
}