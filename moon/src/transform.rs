//! The [`Transform`] ans [`Transform2D`] structs.

use crate::Mat4;
use crate::Vec2;
use crate::Vec3;

/// A 3D representation of an entity's position, rotation and scale.
///
/// A [`Transform`] contains [`Vec3`]s for Position, ROtation and Scale.
#[derive(Debug)]
pub struct Transform {
    matrix: Mat4,
    /// Position component of the [`Transform`].
    pub position: Vec3,
    /// Rotation component of the [`Transform`].
    pub rotation: Vec3,
    /// Scale component of the [`Transform`].
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
    /// Create a new [`Transform`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`Transform`] with an initial position.
    pub fn new_with_position(position: Vec3) -> Self {
        Self {
            position,
            matrix: Mat4::new_translation(&position),
            ..Self::default()
        }
    }

    /// Get a clone of the [`Matrix4`].
    pub fn matrix(&self) -> Mat4 {
        self.matrix
    }

    /// Get the [`Matrix4`] representing the transform as a slice of [`f32`] to use with WebGL.
    pub fn matrix_slice(&mut self) -> &[f32] {
        self.matrix.as_slice()
    }

    /// Recalculate the matrix of the [`Transform`].
    fn recalculate_matrix(&mut self) {
        self.matrix = Mat4::new_translation(&self.position)
            * Mat4::new_rotation(self.rotation)
            * Mat4::new_nonuniform_scaling(&self.scale);
    }

    /// Set the [`Transform`]'s position and calculate its matrix.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.recalculate_matrix();
    }

    /// Set the [`Transform`]'s rotation and calculate its matrix.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = Vec3::z() * rotation;
        self.recalculate_matrix();
    }

    /// Set the [`Transform`]'s scale and calculate its matrix.
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.recalculate_matrix();
    }

    /// Get the position as a slice.
    pub fn get_position(&self) -> &[f32] {
        self.position.as_slice()
    }

    /// Get the rotation as an f32.
    pub fn get_rotation(&self) -> f32 {
        self.rotation.z
    }

    /// Get the scale as a slice.
    pub fn get_scale(&self) -> &[f32] {
        self.scale.as_slice()
    }
}

/// A 2D counterpart for the [`Transform`].
///
/// A [`Transform2D`] contains Position and Scale [`Vec2`]s and a float for rotation.
pub struct Transform2D {
    /// Position of the [`Transform2D`].
    pub position: Vec2,
    /// Rotation of the [`Transform2D`].
    pub rotation: f32,
    /// Scale of the [`Transform2D`].
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
