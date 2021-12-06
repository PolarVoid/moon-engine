use crate::transform::Transform;
use nalgebra::{Vector3, Orthographic3};

const FIXED_WIDTH: f32 = 5.0;
const HEIGHT: f32 = FIXED_WIDTH/1.77;

#[allow(dead_code)]
pub struct Camera {
    pub transform: Transform,
    pub orthographic: Orthographic3<f32>,
    pub width: f32,
    pub height: f32,
    znear: f32,
    zfar: f32,
}

#[allow(dead_code)]
impl Camera {
    /// Create a new `Camera` with default values.
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            width: 1920.0,
            height: 1080.0,
            znear: 0.0f32,
            zfar: 1000.0f32,
            orthographic: Orthographic3::new(-FIXED_WIDTH, FIXED_WIDTH, -HEIGHT, HEIGHT, 0f32, 1000.0f32),
        }
    }
    /// Create a new `Camera` with an initial position.
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            transform: Transform::new_with_position(position),
            ..Camera::new()
        }
    }
    /// Create a new `Camera` with an initial position.
    pub fn with_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Camera::new()
        }
    }
    /// Create a new `Camera` with an initial width and height.
    pub fn with_width_and_height(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            //orthographic: Orthographic3::new(0.0, width, 0.0, height, 0f32, 1000.0f32),
            ..Camera::new()
        }
    }
    /// Set the width and height of the camera plane, and update the Projection Matrix to match.
    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        // self.orthographic.set_right(width);
        // self.orthographic.set_top(height);
    }
    /// Return the Projection Matrix of the `Camera` as a slice of `f32` so it can be used by WebGL.
    pub fn projection(&self) -> &[f32] {
        self.orthographic.as_matrix().as_slice()
    }
}
