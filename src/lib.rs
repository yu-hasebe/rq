use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let json = fetch_json("Sprite-0001.json").await.unwrap();
        let sheet: Sheet = serde_wasm_bindgen::from_value(json).unwrap();

        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);
        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(())).unwrap();
            }
        });
        let error_callback = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err)).unwrap();
            }
        });
        let image = web_sys::HtmlImageElement::new().unwrap();
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        image.set_src("Sprite-0001.png");
        let _ = success_rx.await.unwrap();

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
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        interval_callback.forget();
    });

    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp = resp_value.dyn_into::<web_sys::Response>()?;

    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
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
