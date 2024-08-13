use super::{Point, Rect};
use crate::browser;

use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

pub struct SpriteSheet {
    pub sheet: Option<Sheet>,
    pub image: Option<HtmlImageElement>,
}
// TODO refactor
impl SpriteSheet {
    pub fn draw_sprite(
        &self,
        renderer: &super::Renderer,
        frame_name: &str,
        destination: &Point,
    ) -> Result<()> {
        let cell = self
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(frame_name))
            .ok_or_else(|| anyhow!("invalid frame_name: {}", frame_name))?;
        self.image
            .as_ref()
            .map(|image| {
                renderer.draw_image(
                    image,
                    &Rect {
                        x: cell.frame.x.into(),
                        y: cell.frame.y.into(),
                        w: cell.frame.w.into(),
                        h: cell.frame.h.into(),
                    },
                    &Rect {
                        x: destination.x.into(),
                        y: destination.y.into(),
                        w: cell.frame.w.into(),
                        h: cell.frame.h.into(),
                    },
                )
            })
            .ok_or_else(|| anyhow!("error getting HtmlImageElement"))??;
        Ok(())
    }
}

pub struct JsonAssetLoader {
    jsons: HashMap<String, JsValue>,
}
impl JsonAssetLoader {
    pub fn new() -> Self {
        Self {
            jsons: HashMap::new(),
        }
    }
    pub async fn load_json(&mut self, source: &str) -> Result<JsValue> {
        Ok(self
            .jsons
            .entry(source.to_string())
            .or_insert(browser::fetch_json(source).await?)
            .clone())
    }
    pub async fn load_sheet(&mut self, source: &str) -> Result<Sheet> {
        serde_wasm_bindgen::from_value(self.load_json(source).await?)
            .map_err(|err| anyhow!("error deserializing json: {:#?}", err))
    }
}

pub struct ImageAssetLoader {
    images: HashMap<String, HtmlImageElement>,
}
impl ImageAssetLoader {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
    pub async fn load_image(&mut self, source: &str) -> Result<HtmlImageElement> {
        Ok(self
            .images
            .entry(source.to_string())
            .or_insert(load_image(source).await?)
            .clone())
    }
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}
#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}
#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx
                .send(Ok(()))
                .expect("error sending load image success event");
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx
                .send(Err(anyhow!("error loading image: {:#?}", err)))
                .expect("error sending load image error event");
        }
    });

    let image = browser::new_image()?;
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    let _ = complete_rx.await??;
    Ok(image)
}
