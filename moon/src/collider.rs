//! Definition of the [`Collider`] and [`Collide`] traits, as well as simple Colliders.

use crate::clamp;
use crate::Point;

/// Default bounding box size for a [`Point`].
pub const POINT_BOUNDING_SIZE: f32 = 0.1;

/// The `Collider` trait is implemented by different Colliders.
pub trait Collider {
    /// Get the bounding box of a Collider as an `AABB`.
    fn get_bounding_box(&self) -> AABB;

    /// Get the center of the Collider as a `Point`.
    fn get_center(&self) -> Point;
}

/// The `Collide` trait is used to define collisions between two [Colliders](Collider).
pub trait Collide<T: Collider> {
    /// Checks if two [Colliders](Collider) intersect.
    ///
    /// Returns a `bool` indicating whether a collision occured.
    ///
    /// # Examples
    ///
    /// ```
    /// use moon_engine::Point;
    /// use moon_engine::collider::{Collide, Circle};
    /// let a = Point::new(0.0, 0.5);
    /// let b = Circle::new_size(0.1);
    ///
    /// let collision: bool = a.collide_with(&b);
    /// ```
    fn collide_with(&self, _other: &T) -> bool;
}

/// An Axis-Aligned Bounding Box (AABB).
#[derive(Debug)]
pub struct AABB {
    /// The lowest point on the X and Y axes of the [`AABB`].
    pub min: Point,
    /// The highest point on the X and Y axes of the [`AABB`].
    pub max: Point,
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Point::new(-0.5, -0.5),
            max: Point::new(0.5, 0.5),
        }
    }
}

impl AABB {
    /// Creates a new [`AABB`] with a size of 1 unit, centered at a given co-ordinates.
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            min: Point::new(x - 0.5, y - 0.5),
            max: Point::new(x + 0.5, y + 0.5),
        }
    }
    /// Creates a new [`AABB`] with a given width and height, centered at the origin (0, 0).
    pub fn new_size(width: f32, height: f32) -> Self {
        let half: Point = Point::new(width / 2.0, height / 2.0);
        Self {
            min: -half,
            max: half,
        }
    }
    /// Creates a new [`AABB`] with a given width and height, centered at the given co-ordinates.
    pub fn new_position_and_size(x: f32, y: f32, width: f32, height: f32) -> Self {
        let position: Point = Point::new(x, y);
        let half: Point = Point::new(width / 2.0, height / 2.0);
        Self {
            min: position - half,
            max: position + half,
        }
    }
}

/// A Cicle Collider.
#[derive(Debug)]
pub struct Circle {
    /// The [`Point`] at which the [`Circle`] is centered.
    pub origin: Point,
    /// Radius of the Circle Collider.
    ///
    /// The radius defines how far any given point on it's circumference is from it's center.
    pub radius: f32,
}

impl Circle {
    /// Creates a new [`Circle`] with a radius of 0.5 unit, centered at the given co-ordinates.
    pub fn new_position(x: f32, y: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            radius: 0.5,
        }
    }
    /// Creates a new [`Circle`] with a given radius, centered at the origin (0, 0).
    pub fn new_size(radius: f32) -> Self {
        Self {
            origin: Point::zeros(),
            radius,
        }
    }
    /// Creates a new [`Circle`] with a given radius, centered at the given co-ordinates.
    pub fn new_position_and_size(x: f32, y: f32, radius: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            radius,
        }
    }
}

impl Collider for Point {
    /// Get a bounding box for a [`Point`], using [`POINT_BOUNDING_SIZE`] as its size
    fn get_bounding_box(&self) -> AABB {
        AABB::new_position_and_size(self.x, self.y, POINT_BOUNDING_SIZE, POINT_BOUNDING_SIZE)
    }

    /// Get the center of the [`Point`] Collider
    fn get_center(&self) -> Point {
        *self
    }
}

impl Collider for AABB {
    /// Return the tight bounding box of the `AABB`, as a copy of itself
    fn get_bounding_box(&self) -> AABB {
        AABB { ..*self }
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
        self.min.x < _other.max.x
            && _other.min.x < self.max.x
            && self.min.y < _other.max.y
            && _other.min.y < self.max.y
    }
}

/// AABB and Point Collision
impl Collide<Point> for AABB {
    fn collide_with(&self, _other: &Point) -> bool {
        self.min.x < _other.x
            && _other.x < self.max.x
            && self.min.y < _other.y
            && _other.y < self.max.y
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
