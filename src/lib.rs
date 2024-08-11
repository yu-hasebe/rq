#[macro_use]
mod browser;
mod engine;

use serde::Deserialize;
use std::{collections::HashMap};
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let context = browser::context().expect("no 2d context found");

    browser::spawn_local(async move {
        let json = browser::fetch_json("Sprite-0001.json").await.expect("failed to fetch json");
        let sheet: Sheet = serde_wasm_bindgen::from_value(json).unwrap();
        let image = engine::load_image("Sprite-0001.png").await.expect("failed to load image");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = match (frame + 1) % 4 {
                0 | 2 => 2,
                1 => 3,
                3 => 1,
                _ => panic!(),
            };
            let frame_name = format!("left0{}.png", frame);
            context.clear_rect(0.0, 0.0, 480.0, 480.0);
            let sprite = sheet.frames.get(&frame_name).unwrap();
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sprite.frame.x.into(),
                    sprite.frame.y.into(),
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    160.0,
                    160.0,
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                )
                .unwrap();
        }) as Box<dyn FnMut()>);
        browser::window()
            .expect("no window found")
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        interval_callback.forget();
    });

    Ok(())
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}
