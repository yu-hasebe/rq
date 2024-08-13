use crate::browser;

use anyhow::Result;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

pub const KEY_CODE_ARROW_LEFT: &str = "ArrowLeft";
pub const KEY_CODE_ARROW_UP: &str = "ArrowUp";
pub const KEY_CODE_ARROW_RIGHT: &str = "ArrowRight";
pub const KEY_CODE_ARROW_DOWN: &str = "ArrowDown";

pub fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
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

pub fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
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

pub enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}
pub struct KeyState {
    pressed_keys: HashMap<String, KeyboardEvent>,
}
impl KeyState {
    pub fn new() -> Self {
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

    fn set_pressed(&mut self, code: &str, event: KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }
}
