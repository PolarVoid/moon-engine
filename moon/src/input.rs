
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputManager {
    keyboard_states: HashSet<u8>,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[wasm_bindgen]
impl InputManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            keyboard_states: HashSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
    #[wasm_bindgen]
    pub fn key_down(&mut self, key_code: u8) {
        self.keyboard_states.insert(key_code);
    }
    #[wasm_bindgen]
    pub fn key_up(&mut self, key_code: u8) {
        self.keyboard_states.remove(&key_code);
    }
    #[wasm_bindgen]
    pub fn get_key_state(&self, key_code: u8) -> bool {
        self.keyboard_states.contains(&key_code)
    }
}