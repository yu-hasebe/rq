use crate::browser;
use crate::engine;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

pub struct RQ {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            image: None,
            sheet: None,
            frame: 0,
        }
    }
}

#[async_trait(?Send)]
impl engine::Game for RQ {
    async fn initialize(&self) -> Result<Box<dyn engine::Game>> {
        let json = browser::fetch_json("Sprite-0001.json").await?;
        let sheet: Sheet = serde_wasm_bindgen::from_value(json)
            .map_err(|err| anyhow!("error deserializing json: {:#?}", err))?;
        let image = engine::load_image("Sprite-0001.png").await?;
        Ok(Box::new(RQ {
            image: Some(image),
            sheet: Some(sheet),
            frame: 0,
        }))
    }
    fn update(&mut self) {
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }
    fn draw(&self, renderer: &engine::Renderer) {
        renderer.clear(&engine::Rect {
            x: 0,
            y: 0,
            w: 480,
            h: 480,
        });

        // it is assumed that 0 <= self.frame < 24
        let frame = match self.frame / 6 {
            0 | 2 => 2,
            1 => 3,
            3 => 1,
            _ => panic!(),
        };
        let frame_name = format!("left0{}.png", frame);
        let sprite = self.sheet.as_ref().and_then(|sheet| sheet.frames.get(&frame_name)).unwrap();
        self.image.as_ref().map(|image| {
            renderer.draw_image(
                image,
                &engine::Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    w: sprite.frame.w.into(),
                    h: sprite.frame.h.into(),
                },
                &engine::Rect {
                    x: 240,
                    y: 240,
                    w: sprite.frame.w.into(),
                    h: sprite.frame.h.into(),
                },
            )
        }).unwrap();
    }
}

// TODO move to engine module
#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

// TODO move to engine module
#[derive(Deserialize)]
struct SheetRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

// TODO move to engine module
#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}
