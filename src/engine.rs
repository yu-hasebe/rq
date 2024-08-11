use crate::browser;

use anyhow::{anyhow, Result};
use std::{rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let (complete_tx, complete_rx) = futures::channel::oneshot::channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(())).unwrap();
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx.send(Err(anyhow!("error loading image: {:#?}", err))).unwrap();
        }
    });

    let image = browser::new_image()?;
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    let _ = complete_rx.await??;
    Ok(image)
}
