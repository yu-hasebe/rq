use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;
use anyhow::{Result, anyhow};

pub struct SpriteSheetStore {
    sprite_sheets: HashMap<String, SpriteSheet>,
}
impl SpriteSheetStore {
    pub fn new() -> Self {
        Self {
            sprite_sheets: HashMap::new(),
        }
    }
    pub fn add(&mut self, sprite_sheet: SpriteSheet) -> Result<()> {
        match self.sprite_sheets.insert(sprite_sheet.name.clone(), sprite_sheet) {
            Some(sprite_sheet) => Err(anyhow!("SpriteSheet with the same key has been inserted: {}", sprite_sheet.name)),
            None => Ok(()),
        }
    }

    pub fn get(&self, name: &str) -> Result<&SpriteSheet> {
        self.sprite_sheets.get(name).ok_or_else(|| anyhow!("no SpriteSheet found with name {}", name))
    }
}

pub struct SpriteSheet {
    pub name: String,
    pub sheet: Option<Sheet>,
    pub image: Option<HtmlImageElement>,
}
#[derive(Deserialize)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}
#[derive(Deserialize)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}
#[derive(Deserialize)]
pub struct Cell {
    pub frame: SheetRect,
}
