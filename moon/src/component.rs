//! The [`Component`] trait.

/// The [`Component`] trait
pub trait Component {
    /// Initializes a [`Component`].
    fn init(&mut self) {}
    /// Called once each frame.
    fn update(&mut self, _delta_time: f32) {}
}