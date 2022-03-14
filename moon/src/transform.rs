use crate::Mat4;
use crate::Vec2;
use crate::Vec3;

#[derive(Debug)]
pub struct Transform {
    pub matrix: Mat4,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Mat4::identity(),
            position: Vec3::zeros(),
            rotation: Vec3::zeros(),
            scale: Vec3::from_element(1.0),
        }
    }
}

impl Transform {
    /// Create a new `Transform` with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a new `Transform` with an initial position.
    pub fn new_with_position(position: Vec3) -> Self {
        Self {
            position,
            matrix: Mat4::new_translation(&position),
            ..Self::default()
        }
    }
    /// Get the `Matrix4` representing the transform as a slice of `f32` to use with WebGL.
    pub fn matrix(&mut self) -> &[f32] {
        self.matrix.as_slice()
    }

    fn recalculate_matrix(&mut self) {
        self.matrix = Mat4::new_translation(&self.position)
            * Mat4::new_rotation(self.rotation)
            * Mat4::new_nonuniform_scaling(&self.scale);
    }

    /// Set the positon of the Transform
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.recalculate_matrix();
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = Vec3::z() * rotation;
        self.recalculate_matrix();
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.recalculate_matrix();
    }

    /// Get the position as a slice
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }

    /// Get the rotation as a f32
    pub fn get_rotation(&self) -> f32 {
        self.rotation.z
    }

    /// Get the scale as a slice
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
}

pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::zeros(),
            rotation: 0.0f32,
            scale: Vec2::from_element(1.0),
        }
    }
}
