use nalgebra::{Vector4, Vector2};

use crate::transform::Transform2D;
pub enum ParticleValue<T: Copy> {
    Constant(T),
    Linear(T, T),
    Random(T, T),
}


impl <T: Copy> ParticleValue<T> {
    pub fn get_value(&self, _time: f32) -> T {
        match self {
            ParticleValue::Constant(value) => *value,
            ParticleValue::Linear(min, max) => *max,
            ParticleValue::Random(min, max) => *min,
        }
    }
}

pub struct ParticleProps {
    pub lifetime: ParticleValue<f32>,
    pub velocity: ParticleValue<Vector2<f32>>,
    pub color: ParticleValue<Vector4<f32>>,
    pub size: ParticleValue<f32>,
}

pub struct Particle {
    transform: Transform2D,
    properties: ParticleProps,
    age: f32,
}

impl <T: Default + Copy> Default for ParticleValue<T> {
    fn default() -> Self {
        ParticleValue::Constant(T::default())
    }
}

impl Default for ParticleProps {
    fn default() -> Self {
        Self { 
            lifetime: ParticleValue::Constant(10.0), 
            velocity: ParticleValue::Constant(Vector2::new(0.0, -1.0)), 
            color: ParticleValue::Constant(Vector4::new(1.0, 1.0, 1.0, 1.0)), 
            size: ParticleValue::Constant(1.0)
        }
    }
}