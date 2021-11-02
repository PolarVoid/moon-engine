use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputManager {
    observers: Vec<f32>,
}

#[wasm_bindgen]
impl InputManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}