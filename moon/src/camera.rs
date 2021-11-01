use std::f32::consts;
use nalgebra as na;
use na::{Perspective3, Vector3, Matrix4};

pub struct Camera {
    pub view: Matrix4<f32>,
    position: Vector3<f32>,
    orientation: Vector3<f32>,
    up: Vector3<f32>,
    width: u32,
    height: u32,
    fov: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view: Matrix4::identity(),
            position: Vector3::zeros(),
            orientation: -Vector3::z(),
            up: Vector3::y(),
            width: 1920,
            height: 1080,
            fov: consts::PI/4.0,
            znear: 0.3f32,
            zfar: 1000.0f32,
        }
    }
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            position,
            view: Matrix4::new_translation(&position),
            ..Camera::new()
        }
    }
    pub fn with_width_and_height(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Camera::new()
        }
    }
    pub fn calculate_view(&mut self) {
        self.view = Matrix4::new_translation(&self.position);
    }
    pub fn projection(&self) -> Perspective3<f32> {
        Perspective3::new(self.width as f32 / self.height as f32, self.fov, self.znear, self.zfar)
    }
}