use nalgebra::Vector2;

const POINT_BOUNDING_SIZE: f32 = 0.1;

pub trait Collider {
    fn get_bounding_box(&self) -> AABB;
}

pub trait Collide<T: Collider> {
    fn collide_with(&self, _other: &T) -> bool;
}

#[derive(Debug, Default)]
// Point
pub struct Point {
    pub coords: Vector2<f32>,
}

impl Point {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            coords: Vector2::new(x, y),
        }
    }
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
    pub fn new_size(width: f32, height: f32) -> Self {
        let half: Vector2<f32> = Vector2::new(width / 2.0, height / 2.0);
        Self {
            min: -half,
            max: half,
        }
    }
    pub fn new_position_and_size(x: f32, y: f32, width: f32, height: f32) -> Self {
        let position: Vector2<f32> = Vector2::new(x, y);
        let half: Vector2<f32> = Vector2::new(width / 2.0, height / 2.0);
        Self {
            min: position - half,
            max: position + half,
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
    pub fn new_size(radius: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            radius
        }
    }
    pub fn new_position_and_size(x: f32, y: f32, radius: f32) -> Self {
        Self {
            origin: Vector2::new(x, y),
            radius
        }
    }
}

impl Collider for Point {
    fn get_bounding_box(&self) -> AABB {
        AABB::new_position_and_size(self.coords.x, self.coords.y, POINT_BOUNDING_SIZE, POINT_BOUNDING_SIZE)
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

impl Collide<Point> for Point {
    fn collide_with(&self, _other: &Point) -> bool {
        self.coords == _other.coords
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
        self.min.x < _other.coords.x && _other.coords.x < self.max.x &&
            self.min.y < _other.coords.y && _other.coords.y < self.max.y
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

impl Collide<Point> for Circle {
    fn collide_with(&self, _other: &Point) -> bool {
        let distance: Vector2<f32> = _other.coords - self.origin;
        distance.norm_squared() < self.radius * self.radius
    }
}

impl Collide<AABB> for Circle {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}

