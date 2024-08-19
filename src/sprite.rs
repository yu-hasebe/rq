pub mod player_sprite;

use std::{cell::Ref, ops::Deref};

use crate::engine::Rect;

pub trait Sprite {
    fn source_image(&self) -> String; // TODO enumや型情報にしたい
    fn frame(&self) -> Rect;
    fn destination(&self) -> Rect;
}

pub struct ConverterIntoSprite<'a, T> {
    pub value: Ref<'a, T>,
}

impl<'b, T> Deref for ConverterIntoSprite<'b, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}