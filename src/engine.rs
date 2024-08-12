use crate::browser;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct SpriteSheet {
    pub sheet: Option<Sheet>,
    pub image: Option<HtmlImageElement>,
}
impl SpriteSheet {
    pub fn draw_sprite(
        &self,
        renderer: &Renderer,
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

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[derive(Deserialize)]
struct Cell {
    pub frame: SheetRect,
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let (complete_tx, complete_rx) = futures::channel::oneshot::channel::<Result<()>>();
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

pub struct Renderer {
    context: CanvasRenderingContext2d,
}
impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context
            .clear_rect(rect.x.into(), rect.y.into(), rect.w.into(), rect.h.into());
    }

    pub fn draw_image(
        &self,
        image: &HtmlImageElement,
        frame: &Rect,
        destination: &Rect,
    ) -> Result<()> {
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
            .map_err(|err| anyhow!("error drawing image: {:#?}", err))?;
        Ok(())
    }
}
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

pub const KEY_CODE_ARROW_LEFT: &str = "ArrowLeft";
pub const KEY_CODE_ARROW_UP: &str = "ArrowUp";
pub const KEY_CODE_ARROW_RIGHT: &str = "ArrowRight";
pub const KEY_CODE_ARROW_DOWN: &str = "ArrowDown";

fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);

    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode))
            .expect("error sending keydown event");
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode))
            .expect("error sending keyup event");
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();
    Ok(keyevent_receiver)
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}
pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}
impl KeyState {
    fn new() -> Self {
        Self {
            pressed_keys: HashMap::new(),
        }
    }

    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, key_state: &KeyState) -> Result<()>;
    fn draw(&self, renderer: &Renderer) -> Result<()>;
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;
impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut game = game.initialize().await?;
        let mut game_loop = Self {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };
        let renderer = Renderer {
            context: browser::context()?,
        };
        let mut key_state = KeyState::new();
        let mut keyevent_receiver = prepare_input()?;

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = Rc::clone(&f);
        *g.borrow_mut() = Some(browser::create_request_animation_frame_closure(
            move |perf| {
                process_input(&mut key_state, &mut keyevent_receiver);
                game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
                while game_loop.accumulated_delta > FRAME_SIZE {
                    game.update(&key_state).expect("error GameLoop update");
                    game_loop.accumulated_delta -= FRAME_SIZE;
                }
                game_loop.last_frame = perf;
                game.draw(&renderer).expect("error GameLoop draw");
                browser::request_animation_frame(
                    f.borrow()
                        .as_ref()
                        .expect("error borrowing f ShareLoopClosure"),
                )
                .expect("error request animation frame");
            },
        ));
        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .expect("error borrowing g ShareLoopClosure"),
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
