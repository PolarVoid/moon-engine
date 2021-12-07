use nalgebra::Vector2;

pub trait Collider {
}

pub trait Collide<T: Collider> {
    fn collide_with(&self, _other: &T) -> bool;
}

// Axis-Aligned Bounding Box
pub struct AABB {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Collider for AABB {
    
}

impl Collide<AABB> for AABB {
    fn collide_with(&self, _other: &AABB) -> bool {
        self.min.x < _other.min.x + _other.max.x &&
            _other.min.x < self.min.x + self.max.x &&
            self.min.x < _other.min.x + _other.max.x &&
            _other.min.x < self.min.x + self.max.x
    }
}