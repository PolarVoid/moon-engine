use nalgebra::Vector3;
use nalgebra::Matrix4;

pub struct Transform {
    matrix: Matrix4<f32>,
    position: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::identity(),
            position: Vector3::zeros(),
            scale: Vector3::from_element(1.0),
        }
    }
    pub fn new_with_position(position: Vector3<f32>) -> Self {
        Self {
            matrix: Matrix4::new_translation(&position),
            position,
            ..Transform::new()
        }
    }
    pub fn matrix(&self) -> &[f32] {
        self.matrix.as_slice()
    }
    pub fn translate(&mut self, shift: &Vector3<f32>) {
        self.matrix.append_translation_mut(shift);
    }
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
}