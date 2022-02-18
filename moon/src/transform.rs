use nalgebra::Matrix4;
use nalgebra::Vector2;

pub struct Transform {
    matrix: Matrix4<f32>,
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Transform {
    /// Create a new `Transform` with default values.
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::identity(),
            position: Vector2::zeros(),
            rotation: 0.0f32,
            scale: Vector2::from_element(1.0),
        }
    }
    /// Create a new `Transform` with an initial position.
    pub fn new_with_position(position: Vector2<f32>) -> Self {
        Self {
            position,
            matrix: Matrix4::identity(),
            ..Transform::new()
        }
    }
    /// Get the `Matrix4` representing the transform as a slice of `f32` to use with WebGL.
    pub fn matrix(&mut self) -> &[f32] {
        self.matrix.as_slice()
    }
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
}
