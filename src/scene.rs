pub mod map_scene;

use crate::{
    engine::{Game, KeyState, Rect, Renderer},
    object::player::Player,
};
use map_scene::MapScene;

use anyhow::Result;
use async_trait::async_trait;
use std::{cell::RefCell, rc::Rc};

const CANVAS_WIDTH: i16 = 480;
const CANVAS_HEIGHT: i16 = 480;
pub const TILE_WIDTH: i16 = 32;
pub const TILE_HEIGHT: i16 = 32;

pub struct SceneManager {
    current_scene: SceneEnum,
}
impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: SceneEnum::MapScene(MapScene {
                player: Rc::new(RefCell::new(Player::new(0, 0, 0, 4))),
            }),
        }
    }
}
pub trait Scene {
    fn update(&mut self, key_state: &KeyState) -> Result<()>;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}
#[derive(Clone)]
pub enum SceneEnum {
    MapScene(MapScene),
}

#[async_trait(?Send)]
impl Game for SceneManager {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        Ok(Box::new(SceneManager::new()))
    }
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        match self.current_scene.clone() {
            SceneEnum::MapScene(mut scene) => scene.update(key_state),
        }
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            w: CANVAS_WIDTH,
            h: CANVAS_HEIGHT,
        });
        match self.current_scene.clone() {
            SceneEnum::MapScene(scene) => scene.draw(renderer),
        }
    }
}
