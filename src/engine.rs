use crate::browser;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::{cell::RefCell, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

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
            error_tx
                .send(Err(anyhow!("error loading image: {:#?}", err)))
                .unwrap();
        }
    });

    let image = browser::new_image()?;
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    let _ = complete_rx.await??;
    Ok(image)
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}
impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context
            .clear_rect(rect.x.into(), rect.y.into(), rect.w.into(), rect.h.into());
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                frame.x.into(),
                frame.y.into(),
                frame.w.into(),
                frame.h.into(),
                destination.x.into(),
                destination.y.into(),
                destination.w.into(),
                destination.h.into(),
            )
            .unwrap();
    }
}
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, renderer: &Renderer);
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;
impl GameLoop {
    pub async fn start(mut game: impl Game + 'static) -> Result<()> {
        let mut game = game.initialize().await?;
        let mut game_loop = Self {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };
        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);
        *g.borrow_mut() = Some(browser::create_request_animation_frame_closure(
            move |perf| {
                game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
                while game_loop.accumulated_delta > FRAME_SIZE {
                    game.update();
                    game_loop.accumulated_delta -= FRAME_SIZE;
                }
                game_loop.last_frame = perf;
                game.draw(&renderer);
                browser::request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
            },
        ));
        browser::request_animation_frame(g.borrow().as_ref().unwrap()).unwrap();
        Ok(())
    }
}
