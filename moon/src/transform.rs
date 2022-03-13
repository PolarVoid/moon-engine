use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Transform {
    pub matrix: Matrix4<f32>,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Matrix4::identity(),
            position: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scale: Vector3::from_element(1.0),
        }
    }
}

impl Transform {
    /// Create a new `Transform` with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a new `Transform` with an initial position.
    pub fn new_with_position(position: Vector3<f32>) -> Self {
        Self {
            position,
            matrix: Matrix4::new_translation(&position),
            ..Self::default()
        }
    }
    /// Get the `Matrix4` representing the transform as a slice of `f32` to use with WebGL.
    pub fn matrix(&mut self) -> &[f32] {
        self.matrix.as_slice()
    }

    fn recalculate_matrix(&mut self) {
        self.matrix = Matrix4::new_translation(&self.position)
            * Matrix4::new_rotation(self.rotation)
            * Matrix4::new_nonuniform_scaling(&self.scale);
    }

    /// Set the positon of the Transform
    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.recalculate_matrix();
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = Vector3::z() * rotation;
        self.recalculate_matrix();
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
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
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vector2::zeros(),
            rotation: 0.0f32,
            scale: Vector2::from_element(1.0),
        }
    }
}
