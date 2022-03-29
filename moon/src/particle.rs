//! The [`Particle`] and [`ParticleSystem`] structs.

use crate::component::Component;
use crate::math::*;
use crate::renderer::Quad;
use crate::transform::Transform2D;

/// Maximum [`Particles`](Particle) in a [`ParticleSystem`].
const MAX_PARTICLES: usize = 10000;

/// A [`ParticleProps`] defines how [`Particles`](Particle) are created.
///
/// Reusing a [`ParticleProps`] allows for similar [`Particles`](Particle) to be emitted.
#[derive(Debug, Clone)]
pub struct ParticleProps {
    /// How long the [`Particle`] will last.
    pub lifetime: f32,
    /// The base velocity of the [`Particle`].
    pub velocity: Vec2,
    /// A modifier field for the velocity of the [`Particle`].
    pub velocity_modifier: Vec2,
    /// The start color of the [`Particle`].
    pub color_start: Color32,
    /// The end color of the [`Particle`].
    pub color_end: Color32,
    /// A modifier field for the color of the [`Particle`].
    pub color_modifier: Color32,
    /// The size of the [`Particle`].
    pub size: Vec2,
}

impl Default for ParticleProps {
    fn default() -> Self {
        Self {
            lifetime: 10.0,
            velocity: Vec2::new(0.0, -0.005),
            velocity_modifier: Vec2::new(0.002, 0.003),
            color_start: Color32(1.0, 1.0, 0.0, 1.0),
            color_end: Color32(1.0, 0.0, 0.0, 1.0),
            color_modifier: Color32::ZEROES,
            size: Vec2::from_element(0.05),
        }
    }
}

/// A [`Particle`] describes a single emission from a [`ParticleSystem`].
#[derive(Debug, Clone)]
pub struct Particle {
    transform: Transform2D,
    lifetime: f32,
    velocity: Vec2,
    color: Color32,
    color_start: Color32,
    color_end: Color32,
    age: f32,
    alive: bool,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            transform: Transform2D::new_with_scale(0.1, 0.1),
            lifetime: 10.0,
            velocity: Vec2::new(0.0, 0.0),
            color: Color32::ZEROES,
            color_start: Color32::WHITE,
            color_end: Color32::WHITE,
            age: 0.0,
            alive: false,
        }
    }
}

impl Component for Particle {
    fn init(&mut self) {
        self.color = self.color_start;
        self.alive = true;
        self.age = 0.0;
    }

    fn update(&mut self, delta_time: f32) {
        self.age += delta_time;
        if self.age > self.lifetime {
            self.alive = false;
        } else {
            self.transform.position += self.velocity;
            self.color = Color32::lerp(self.color_start, self.color_end, self.age / self.lifetime);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl From<&ParticleProps> for Particle {
    fn from(properties: &ParticleProps) -> Self {
        Self {
            transform: Transform2D::new_with_scale(properties.size.x, properties.size.y),
            lifetime: properties.lifetime,
            velocity: { properties.velocity + Vec2::random_range(-properties.velocity_modifier, properties.velocity_modifier) },
            color_start: properties.color_start + Color32::random_range(properties.color_modifier, properties.color_modifier),
            color_end: properties.color_end + Color32::random_range(properties.color_modifier, properties.color_modifier),
            ..Default::default()
        }
    }
}

/// A [`ParticleSystem`] deals with the emission, and creation of [`Particles`](Particle).
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    emission: ParticleProps,
    particles: Vec<Particle>,
    index: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            emission: ParticleProps::default(),
            particles: Vec::with_capacity(MAX_PARTICLES),
            index: 0,
        }
    }
}

impl Component for ParticleSystem {
    fn init(&mut self) {
        self.particles.fill_with(|| Particle::default());
    }

    fn update(&mut self, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            if particle.alive {
                particle.update(delta_time);
            }
        }
    }
    /// Get a [`Vec`] of [`Quad`] from all the [`Particles`](Particle).
    fn get_quads(&self) -> Option<Vec<Quad>> {
        Some(
            self.particles.iter()
                .filter_map(|particle| 
                    if particle.alive {
                        Some(particle)
                    } else {
                        None
                    }).map(|particle| {
                        Quad::new_from_position_and_size_and_color(
                            particle.transform.position.x, 
                            particle.transform.position.y, 
                            particle.transform.scale.x, 
                            particle.transform.scale.y,
                            particle.color
                        )
                    }).collect()
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ParticleSystem {
    /// Create a new [`ParticleSystem`] using a [`ParticleProps`] for the emission.
    pub fn new_from_emission(emission: ParticleProps) -> Self {
        Self {
            emission,
            ..Default::default()
        }
    }

    /// Emit a single [`Particle`], according to the defined [`ParticleProps`] for emission.
    pub fn emit(&mut self) {
        if self.index >= MAX_PARTICLES {
            self.index = 0;
        }

        let mut new_particle = Particle::from(&self.emission);

        let particle = self.particles.get_mut(self.index);

        if let Some(particle) = particle {
            particle.transform = new_particle.transform;
            particle.lifetime = new_particle.lifetime;
            particle.velocity = new_particle.velocity;
            particle.color_start = new_particle.color_start;
            particle.color_end = new_particle.color_end;

            particle.init();
        } else {
            new_particle.init();
            self.particles.push(new_particle);
        }
        self.index += 1;
    }

    /// Emit multiple [`Particles`](Particle), according to the defined [`ParticleProps`] for emission.
    pub fn emit_many(&mut self, count: u32) {
        for _ in 0..count {
            self.emit()
        }
    }
}
