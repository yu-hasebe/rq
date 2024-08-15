mod player;

use crate::browser;
use crate::engine::{
    Game, GameObject as _, ImageAssetLoader, JsonAssetLoader, KeyState, Rect, Renderer, Sheet, SpriteSheet
};

use anyhow::Result;
use async_trait::async_trait;
use player::Player;

const CANVAS_WIDTH: i16 = 480;
const CANVAS_HEIGHT: i16 = 480;
const TILE_WIDTH: i16 = 32;
const TILE_HEIGHT: i16 = 32;

pub struct RQ {
    frame: u8,
    player: Player,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            frame: 0,
            player: Player::new("Sprite-0001", None),
        }
    }
}

#[async_trait(?Send)]
impl Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        let mut json_asset_loader = JsonAssetLoader::new();
        let mut image_asset_loader = ImageAssetLoader::new();
        let json = json_asset_loader.load_json("Sprite-0001.json").await?;
        let sheet: Sheet = browser::from_value(json)?;
        let image = image_asset_loader.load_image("Sprite-0001.png").await?;
        Ok(Box::new(Self {
            frame: self.frame,
            player: Player::new(
                "Sprite-0001",
                Some(SpriteSheet {
                    sheet: Some(sheet),
                    image: Some(image),
                }),
            ),
        }))
    }
    fn update(&mut self, key_state: &KeyState) -> Result<()> {
        self.player.update(key_state)
    }
    fn draw(&self, renderer: &Renderer) -> Result<()> {
        renderer.clear(&Rect {
            x: 0,
            y: 0,
            w: CANVAS_WIDTH,
            h: CANVAS_HEIGHT,
        });
        self.player.draw(renderer)
    }
}
