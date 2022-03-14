use std::collections::BTreeSet;

#[derive(Default)]
pub struct InputManager {
    keyboard_states: BTreeSet<u8>,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl InputManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn key_down(&mut self, key_code: u8) {
        self.keyboard_states.insert(key_code);
    }

    pub fn key_up(&mut self, key_code: u8) {
        self.keyboard_states.remove(&key_code);
    }

    pub fn get_key_state(&self, key_code: u8) -> bool {
        self.keyboard_states.contains(&key_code)
    }
}
