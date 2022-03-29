//! The [`Component`] trait.

use std::any::Any;

use crate::{renderer::Quad, Mat4};

/// The [`Component`] trait
pub trait Component {
    /// Initializes a [`Component`].
    fn init(&mut self) {}
    /// Called once each frame.
    ///
    /// The time between frames is also provided.
    fn update(&mut self, _delta_time: f32) {}

    /// Get the [`Component`]'s drawable [`Quad`].
    fn get_quads(&self) -> Option<Vec<Quad>> {
        None
    }

    /// Move the [`Component`] by the given `X` and `Y` deltas.
    fn translate(&mut self, _pos_x: f32, _pos_y: f32) {}

    /// Get the model matrix for the [`Component`].
    fn get_matrix(&self) -> Mat4 {
        Mat4::identity()
    }

    /// Borrow the [`Component`] as a [`&mut dyn Any`](Any).
    fn as_any(&self) -> &dyn Any;

    /// Borrow the [`Component`] as a [`&dyn Any`](Any).
    fn as_mut_any(&mut self) -> &mut dyn Any;
}
