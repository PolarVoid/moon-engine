use nalgebra::{Vector2, clamp};

/// Default bounding box size for a `Point`
const POINT_BOUNDING_SIZE: f32 = 0.1;

/// The `Collider` trait is implemented by different Colliders
pub trait Collider {
    /// Get the bounding box of a Collider as an `AABB`
    fn get_bounding_box(&self) -> AABB;

    /// Get the center of the Collider as a `Point`
    fn get_center(&self) -> Point;
}

/// The `Collide` trait is used to define collisions between two Colliders
pub trait Collide<T: Collider> {
    fn collide_with(&self, _other: &T) -> bool;
}

/// A `Point` is an alias for `nalgebra::Vector2<f32>`
pub type Point = Vector2<f32>;

/// Axis-Aligned Bounding Box (AABB)
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

/// Cicle
#[derive(Debug, Default)]
pub struct Circle {
    pub origin: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            radius: 0.5,
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
    /// Get a bounding box for a `Point`, using `POINT_BOUNDING_SIZE` as its size
    fn get_bounding_box(&self) -> AABB {
        AABB::new_position_and_size(self.x, self.y, POINT_BOUNDING_SIZE, POINT_BOUNDING_SIZE)
    }

    /// Get the center of the Point Collider
    fn get_center(&self) -> Point {
        *self
    }
}

impl Collider for AABB {
    /// Return the tight bounding box of the `AABB`, as a copy of itself
    fn get_bounding_box(&self) -> AABB {
        AABB { 
            ..*self
        }
    }

    /// Get the center of the AABB Collider
    fn get_center(&self) -> Point {
        self.max - self.min
    }
}

impl Collider for Circle {
    fn get_bounding_box(&self) -> AABB {
        AABB { 
            min: self.origin - Point::from_element(1.0),
            max: self.origin + Point::from_element(1.0),
        }
    }

    fn get_center(&self) -> Point {
        self.origin
    }
}

/// Point and Point Collsion
impl Collide<Point> for Point {
    fn collide_with(&self, _other: &Point) -> bool {
        self == _other
    }
}

/// Point and AABB Collison
impl Collide<AABB> for Point {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}

/// Point and Circle Collison
impl Collide<Circle> for Point {
    fn collide_with(&self, _other: &Circle) -> bool {
        _other.collide_with(self)
    }
}

/// AABB and AABB Collison
impl Collide<AABB> for AABB {
    fn collide_with(&self, _other: &AABB) -> bool {
        self.min.x < _other.max.x && _other.min.x < self.max.x &&
            self.min.y < _other.max.y && _other.min.y < self.max.y
    }
}

/// AABB and Point Collision
impl Collide<Point> for AABB {
    fn collide_with(&self, _other: &Point) -> bool {
        self.min.x < _other.x && _other.x < self.max.x &&
            self.min.y < _other.y && _other.y < self.max.y
    }
}

/// AABB and Circle Collision
impl Collide<Circle> for AABB {
    fn collide_with(&self, _other: &Circle) -> bool {
        let mut closest: Point = _other.origin;
        closest.x = clamp(closest.x, self.min.x, self.max.x);
        closest.y = clamp(closest.y, self.min.y, self.max.y);
        
        _other.collide_with(&closest)
    }
}

/// Circle and Circle Collision
impl Collide<Circle> for Circle {
    fn collide_with(&self, _other: &Circle) -> bool {
        let radii_sum = self.radius + _other.radius;
        let distance: Point = _other.origin - self.origin;
        distance.norm_squared() < radii_sum * radii_sum
    }
}

/// Circle and Point Collision
impl Collide<Point> for Circle {
    fn collide_with(&self, _other: &Point) -> bool {
        let distance: Point = _other - self.origin;
        distance.norm_squared() < self.radius * self.radius
    }
}

/// Circle and AABB Collision
impl Collide<AABB> for Circle {
    fn collide_with(&self, _other: &AABB) -> bool {
        _other.collide_with(self)
    }
}
