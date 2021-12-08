use nalgebra::{Vector2, clamp};

const POINT_BOUNDING_SIZE: f32 = 0.1;

pub trait Collider {
    fn get_bounding_box(&self) -> AABB;
}

pub trait Collide<T: Collider> {
    fn collide_with(&self, _other: &T) -> bool;
}

pub type Point = Vector2<f32>;

// Axis-Aligned Bounding Box
#[derive(Debug, Default)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            min: Point::new(x - 0.5, y - 0.5),
            max: Point::new(x + 0.5, y + 0.5),
        }
    }
    pub fn new_size(width: f32, height: f32) -> Self {
        let half: Point = Point::new(width / 2.0, height / 2.0);
        Self {
            min: -half,
            max: half,
        }
    }
    pub fn new_position_and_size(x: f32, y: f32, width: f32, height: f32) -> Self {
        let position: Point = Point::new(x, y);
        let half: Point = Point::new(width / 2.0, height / 2.0);
        Self {
            min: position - half,
            max: position + half,
        }
    }
}
#[derive(Debug, Default)]
pub struct Circle {
    pub origin: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            radius: 1.0,
        }
    }
    pub fn new_size(radius: f32) -> Self {
        Self {
            origin: Point::zeros(),
            radius
        }
    }
    pub fn new_position_and_size(x: f32, y: f32, radius: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            radius
        }
    }
}

impl Collider for Point {
    fn get_bounding_box(&self) -> AABB {
        AABB::new_position_and_size(self.x, self.y, POINT_BOUNDING_SIZE, POINT_BOUNDING_SIZE)
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
            min: self.origin - Point::from_element(1.0),
            max: self.origin + Point::from_element(1.0),
        }
    }
}

impl Collide<Point> for Point {
    fn collide_with(&self, _other: &Point) -> bool {
        self == _other
    }
}

impl Collide<AABB> for Point {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}

impl Collide<Circle> for Point {
    fn collide_with(&self, _other: &Circle) -> bool {
        _other.collide_with(self)
    }
}

impl Collide<AABB> for AABB {
    fn collide_with(&self, _other: &AABB) -> bool {
        self.min.x < _other.max.x && _other.min.x < self.max.x &&
            self.min.y < _other.max.y && _other.min.y < self.max.y
    }
}

impl Collide<Point> for AABB {
    fn collide_with(&self, _other: &Point) -> bool {
        self.min.x < _other.x && _other.x < self.max.x &&
            self.min.y < _other.y && _other.y < self.max.y
    }
}

impl Collide<Circle> for AABB {
    fn collide_with(&self, _other: &Circle) -> bool {
        let mut closest: Point = _other.origin;
        closest.x = clamp(closest.x, self.min.x, self.max.x);
        closest.y = clamp(closest.y, self.min.y, self.max.y);
        
        _other.collide_with(&closest)
    }
}

impl Collide<Circle> for Circle {
    fn collide_with(&self, _other: &Circle) -> bool {
        let distance: Point = _other.origin - self.origin;
        distance.norm_squared() < self.radius * _other.radius
    }
}

impl Collide<Point> for Circle {
    fn collide_with(&self, _other: &Point) -> bool {
        let distance: Point = _other - self.origin;
        distance.norm_squared() < self.radius * self.radius
    }
}

impl Collide<AABB> for Circle {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}

