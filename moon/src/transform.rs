use nalgebra::Quaternion;
use nalgebra::Vector3;
use nalgebra::Matrix4;

pub struct Transform {
    matrix: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    /// Create a new `Transform` with default values.
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::identity(),
            position: Vector3::zeros(),
            rotation: Quaternion::identity(),
            scale: Vector3::from_element(1.0),
        }
    }
    /// Create a new `Transform` with an initial position.
    pub fn new_with_position(position: Vector3<f32>) -> Self {
        Self {
            matrix: Matrix4::new_translation(&position),
            position,
            ..Transform::new()
        }
    }
    /// Get the `Matrix4` representing the transform as a slice of `f32` to use with WebGL.
    pub fn matrix(&self) -> &[f32] {
        self.matrix.as_slice()
    }
    /// Translate the position by a given shift value, and update the `Matrix4`.
    pub fn translate(&mut self, shift: &Vector3<f32>) {
        self.position += shift;
        self.matrix.append_translation_mut(shift);
    }
    pub fn rotate(&mut self) {
        
    }
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
}