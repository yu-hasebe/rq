#[macro_use]
mod browser;
mod engine;
mod scene;
mod sprite;
mod object;

use crate::engine::GameLoop;
use crate::scene::SceneManager;
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = SceneManager::new();
        GameLoop::start(game)
            .await
            .expect("error starting GameLoop");
    });

    Ok(())
}
