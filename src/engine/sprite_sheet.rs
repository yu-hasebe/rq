use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

pub struct SpriteSheet {
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
