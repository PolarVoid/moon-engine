use crate::transform::Transform;
use nalgebra::{Perspective3, Vector3};
use std::f32::consts;

#[allow(dead_code)]
pub struct Camera {
    pub transform: Transform,
    pub perspective: Perspective3<f32>,
    orientation: Vector3<f32>,
    up: Vector3<f32>,
    pub width: f32,
    pub height: f32,
    fov: f32,
    znear: f32,
    zfar: f32,
}

#[allow(dead_code)]
impl Camera {
    /// Create a new `Camera` with default values.
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width: 1920.0,
            height: 1080.0,
            fov: consts::PI / 4.0,
            znear: 0.3f32,
            zfar: 1000.0f32,
            perspective: Perspective3::new(192.0 / 108.0, consts::PI / 4.0, 0.3f32, 1000.0f32),
        }
    }
    /// Create a new `Camera` with an initial position.
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            transform: Transform::new_with_position(position),
            ..Camera::new()
        }
    }
    /// Create a new `Camera` with an initial width and height.
    pub fn with_width_and_height(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            perspective: Perspective3::new(width / height, consts::PI / 4.0, 0.3f32, 1000.0f32),
            ..Camera::new()
        }
    }
    /// Set the width and height of the camera plane, and update the Projection Matrix to match.
    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.perspective.set_aspect(width / height);
    }
    /// Return the Projection Matrix of the `Camera` as a slice of `f32` so it can be used by WebGL.
    pub fn projection(&self) -> &[f32] {
        self.perspective.as_matrix().as_slice()
    }
}
