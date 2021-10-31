use nalgebra as na;
use nalgebra::{Matrix4, Vector3, Vector4};

pub struct Camera {
    position: Vector3<f32>,
    orientation: Vector3<f32>,
    up: Vector3<f32>,
    width: u32,
    height: u32,
    speed: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::zeros(),
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width: 1920,
            height: 1080,
            speed: 1.0f32,
            sensitivity: 100.0f32,
        }
    }
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            position,
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width: 1920,
            height: 1080,
            speed: 1.0f32,
            sensitivity: 100.0f32,
        }
    }
    pub fn with_width_and_height(width: u32, height: u32) -> Self {
        Self {
            position: Vector3::zeros(),
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width,
            height,
            speed: 1.0f32,
            sensitivity: 100.0f32,
        }
    }
}