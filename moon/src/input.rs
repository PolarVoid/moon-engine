//! The [`InputManager`] struct.

use std::collections::BTreeSet;

use crate::Vec2;

/// A store for Input-related data.
/// 
/// The [`InputManager`] stores and handles the current input states.
/// 
/// # Examples
/// ```
/// use moon::input::InputManager;
/// 
/// let mut input = InputManager::new();
/// 
/// input.key_down(b'w');
/// 
/// assert!(input.get_key_state(b'w'));
/// ```
#[derive(Default)]
pub struct InputManager {
    /// Set of Keyboard key states.
    /// 
    /// If a key is present, then it is being pressed, and otherwise it is not.
    keyboard_states: BTreeSet<u8>,
    /// Position of the Mouse.
    /// 
    /// The Screen-Space position of the Mouse as a [`Vec2`].
    pub mouse_position: Vec2,
}

impl InputManager {
    /// Default [`InputManager`] instance.
    /// 
    /// Creates a new [`InputManager`] with default keyboard and mouse input states.
    pub fn new() -> Self {
        Default::default()
    }

    /// Key Down State.
    /// 
    /// Sets the key in the [`BTreeSet`].
    pub fn key_down(&mut self, key_code: u8) {
        self.keyboard_states.insert(key_code);
    }

    /// Key Up State.
    /// 
    /// Resets the key in the [`BTreeSet`].
    pub fn key_up(&mut self, key_code: u8) {
        self.keyboard_states.remove(&key_code);
    }

    /// Get the state of a key as a [`bool`].
    /// 
    /// Returns true if the key is currently pressed, or false.
    pub fn get_key_state(&self, key_code: u8) -> bool {
        self.keyboard_states.contains(&key_code)
    }

    /// Set the mouse position.
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position.x = x;
        self.mouse_position.y = y;
    }
}
