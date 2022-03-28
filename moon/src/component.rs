//! The [`Component`] trait.

/// The [`Component`] trait
pub trait Component {
    /// Initializes a [`Component`].
    fn init(&mut self) {}
    /// Called once each frame.
    /// 
    /// The time between frames is also provided.
    fn update(&mut self, _delta_time: f32) {}
}