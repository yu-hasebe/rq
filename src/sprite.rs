pub mod player_sprite;

use crate::engine::Rect;

pub trait Sprite {
    fn source_image(&self) -> String; // TODO enumや型情報にしたい
    fn frame(&self) -> Rect;
    fn destination(&self) -> Rect;
}