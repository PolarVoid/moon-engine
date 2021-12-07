use nalgebra::Vector2;

pub trait Collider {
    fn get_bounding_box(&self) -> AABB;
}

pub trait Collide<T: Collider> {
    fn collide_with(&self, _other: &T) -> bool;
}

// Axis-Aligned Bounding Box
#[derive(Debug, Default)]
pub struct AABB {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl AABB {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            min: Vector2::new(x - 0.5, y - 0.5),
            max: Vector2::new(x + 0.5, y + 0.5),
        }
    }
}
#[derive(Debug, Default)]
pub struct Circle {
    pub origin: Vector2<f32>,
    pub radius: f32,
}

impl Circle {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            origin: Vector2::new(x, y),
            radius: 1.0,
        }
    }
}

impl Collider for AABB {
    fn get_bounding_box(&self) -> AABB {
        AABB { 
            ..*self
        }
    }
}

impl Collider for Circle {
    fn get_bounding_box(&self) -> AABB {
        AABB { 
            min: self.origin - Vector2::from_element(1.0),
            max: self.origin + Vector2::from_element(1.0),
        }
    }
}

impl Collide<AABB> for AABB {
    fn collide_with(&self, _other: &AABB) -> bool {
        self.min.x < _other.max.x && _other.min.x < self.max.x &&
            self.min.y < _other.max.y && _other.min.y < self.max.y
    }
}

impl Collide<Circle> for AABB {
    fn collide_with(&self, _other: &Circle) -> bool {
        todo!()
    }
}

impl Collide<Circle> for Circle {
    fn collide_with(&self, _other: &Circle) -> bool {
        let distance: Vector2<f32> = _other.origin - self.origin;
        distance.norm_squared() < self.radius * _other.radius
    }
}

impl Collide<AABB> for Circle {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}

