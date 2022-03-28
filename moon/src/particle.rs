//! THe [`Particle`] and [`ParticleSystem`].

use crate::math::*;
use crate::component::Component;
use crate::transform::Transform2D;

/// Maximum [`Particles`](Particle) in a [`ParticleSystem`].
const MAX_PARTICLES: usize = 100;

pub struct ParticleProps {
    pub lifetime: f32,
    pub velocity: Vec2,
    pub color: Color32,
    pub size: f32,
}

impl Default for ParticleProps {
    fn default() -> Self {
        Self {
            lifetime: 10.0,
            velocity: Vec2::new(0.0, -1.0),
            color: Color32::WHITE,
            size: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    transform: Transform2D,
    lifetime: f32,
    velocity: Vec2,
    age: f32,
    alive: bool,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            lifetime: 10.0,
            velocity: Vec2::new(0.0, -1.0),
            age: 0.0,
            alive: true,
        }
    }
}

impl Component for Particle {
    fn update(&mut self, delta_time: f32) {
        self.age += delta_time;
        if self.age > self.lifetime {
            self.alive = false;
        } else {
            self.transform.position += self.velocity;
        }
    }
}

/// A [`ParticleSystem`] deals with the emmision, and creation of [`Particles`](Particle).
pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self { 
            particles: Vec::with_capacity(MAX_PARTICLES)
        }
    }
}

impl Component for ParticleSystem {
    fn init(&mut self) {
        self.particles.fill(Particle::default())
    }

    fn update(&mut self, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            if particle.alive {
                particle.update(delta_time);
            }
        }
    }
}