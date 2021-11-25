use nalgebra::UnitQuaternion;
use nalgebra::UnitVector3;
use nalgebra::Vector3;
use nalgebra::Matrix4;

#[allow(dead_code)]
pub struct Transform {
    matrix: Matrix4<f32>,
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    /// Create a new `Transform` with default values.
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::identity(),
            position: Vector3::zeros(),
            rotation: UnitQuaternion::identity(),
            scale: Vector3::from_element(1.0),
        }
    }
    /// Create a new `Transform` with an initial position.
    pub fn new_with_position(position: Vector3<f32>) -> Self {
        Self {
            position,
            matrix: Matrix4::identity(),
            ..Transform::new()
        }
    }
    /// Get the `Matrix4` representing the transform as a slice of `f32` to use with WebGL.
    pub fn matrix(&mut self) -> &[f32] {
        self.matrix = self.rotation.to_homogeneous().prepend_translation(&self.position);
        self.matrix.as_slice()
    }
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
    pub fn front(&self) -> Vector3<f32> {
        self.rotation.transform_vector(&Vector3::z())
    }
    pub fn right(&self) -> Vector3<f32> {
        self.rotation.transform_vector(&Vector3::x())
    }
}