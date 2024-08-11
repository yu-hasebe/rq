use anyhow::{anyhow, Result};
use std::future::Future;
use wasm_bindgen::closure::{WasmClosure, WasmClosureFnOnce};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Window};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("no window found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("no document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("no element with id canvas found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|err| anyhow!("element with id canvas not HtmlCanvasElement: {:#?}", err))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("error getting 2d context: {:#?}", js_value))?
        .ok_or_else(|| anyhow!("no 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|object| {
            anyhow!(
                "error converting {:#?} into CanvasRenderingContext2d",
                object
            )
        })
}

pub fn now() -> Result<f64> {
    Ok(window()?
        .performance()
        .ok_or_else(|| anyhow!("no performance object"))?
        .now())
}

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_json(json_path: &str) -> Result<JsValue> {
    let resp_value = fetch_with_str(json_path).await?;
    let resp = resp_value
        .dyn_into::<web_sys::Response>()
        .map_err(|js_value| anyhow!("error converting into Response: {:#?}", js_value))?;

    wasm_bindgen_futures::JsFuture::from(
        resp.json()
            .map_err(|js_value| anyhow!("error converting into json: {:#?}", js_value))?,
    )
    .await
    .map_err(|js_value| anyhow!("error fetching json: {:#?}", js_value))
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue> {
    let window = web_sys::window().unwrap();
    return wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(resource))
        .await
        .map_err(|js_value| anyhow!("error fetching resource: {:#?}", js_value));
}

pub fn new_image() -> Result<HtmlImageElement> {
    web_sys::HtmlImageElement::new()
        .map_err(|js_value| anyhow!("error creating new HtmlImageElement: {:#?}", js_value))
}

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn closure_wrap<T>(data: Box<T>) -> Closure<T>
where
    T: ?Sized + WasmClosure,
{
    Closure::wrap(data)
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;
pub fn create_request_animation_frame_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    closure_wrap(Box::new(f))
}
pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|js_value| anyhow!("error request animation frame: {:#?}", js_value))
}
