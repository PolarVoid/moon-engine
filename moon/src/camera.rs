use std::f32::consts;
use crate::transform::Transform;
use nalgebra as na;
use na::{Perspective3, Vector3, Matrix4};

#[allow(dead_code)]
pub struct Camera {
    pub transform: Transform,
    pub perspective: Perspective3<f32>,
    orientation: Vector3<f32>,
    up: Vector3<f32>,
    width: u32,
    height: u32,
    fov: f32,
    znear: f32,
    zfar: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width: 1920,
            height: 1080,
            fov: consts::PI/4.0,
            znear: 0.3f32,
            zfar: 1000.0f32,
            perspective: Perspective3::new(192.0/108.0, consts::PI/4.0, 0.3f32, 1000.0f32),
        }
    }
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            transform: Transform::new_with_position(position),
            ..Camera::new()
        }
    }
    pub fn with_width_and_height(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            perspective: Perspective3::new(width as f32 / height as f32, consts::PI/4.0, 0.3f32, 1000.0f32),
            ..Camera::new()
        }
    }
    pub fn projection(&self) -> &[f32] {
        self.perspective.as_matrix().as_slice()
    }
}