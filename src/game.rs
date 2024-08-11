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
    position: engine::Point,
}
impl RQ {
    pub fn new() -> Self {
        Self {
            image: None,
            sheet: None,
            frame: 0,
            position: engine::Point { x: 0, y: 0 },
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
        Ok(Box::new(Self {
            image: Some(image),
            sheet: Some(sheet),
            frame: self.frame,
            position: self.position,
        }))
    }
    fn update(&mut self, key_state: &engine::KeyState) {
        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }

        let mut velocity = engine::Point { x: 0, y: 0 };
        if key_state.is_pressed("ArrowDown") {
            velocity.y += 3;
        }
        if key_state.is_pressed("ArrowUp") {
            velocity.y -= 3;
        }
        if key_state.is_pressed("ArrowRight") {
            velocity.x += 3;
        }
        if key_state.is_pressed("ArrowLeft") {
            velocity.x -= 3;
        }
        self.position.x += velocity.x;
        self.position.y += velocity.y;
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
        let sprite = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .unwrap();
        self.image
            .as_ref()
            .map(|image| {
                renderer.draw_image(
                    image,
                    &engine::Rect {
                        x: sprite.frame.x.into(),
                        y: sprite.frame.y.into(),
                        w: sprite.frame.w.into(),
                        h: sprite.frame.h.into(),
                    },
                    &engine::Rect {
                        x: self.position.x.into(),
                        y: self.position.y.into(),
                        w: sprite.frame.w.into(),
                        h: sprite.frame.h.into(),
                    },
                )
            })
            .unwrap();
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
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

// TODO move to engine module
#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}
