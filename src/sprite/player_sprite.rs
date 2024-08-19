use crate::{
    engine::Rect,
    object::player::{Direction, Player},
    scene::{TILE_HEIGHT, TILE_WIDTH},
};

use super::Sprite;

pub struct PlayerSprite {
    source_image: String,
    frame: Rect,
    destination: Rect,
}

impl From<&Player> for PlayerSprite {
    fn from(from: &Player) -> PlayerSprite {
        let mut frame = match from.direction {
            Direction::Left => Rect {
                x: TILE_WIDTH,
                y: TILE_HEIGHT,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
            Direction::Up => Rect {
                x: TILE_WIDTH,
                y: 3 * TILE_HEIGHT,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
            Direction::Right => Rect {
                x: TILE_WIDTH,
                y: 2 * TILE_HEIGHT,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
            Direction::Down => Rect {
                x: TILE_WIDTH,
                y: 0,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
        };
        match from.frame / 4 {
            1 => frame.x -= TILE_WIDTH,
            3 => frame.x += TILE_WIDTH,
            _ => {}
        }

        PlayerSprite {
            source_image: "Sprite-0001".to_string(),
            frame: frame,
            destination: Rect {
                x: from.pixel_x,
                y: from.pixel_y,
                w: TILE_WIDTH,
                h: TILE_HEIGHT,
            },
        }
    }
}

impl Sprite for PlayerSprite {
    fn source_image(&self) -> String {
        self.source_image.clone()
    }
    fn frame(&self) -> Rect {
        self.frame.clone()
    }
    fn destination(&self) -> Rect {
        self.destination.clone()
    }
}
