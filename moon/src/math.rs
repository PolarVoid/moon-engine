//! Math related functionality, and aliases to [`nalgebra`] structs.
//! 
//! Also includes the [`Color32`] and [`Color8`] structs.

/// An alias to [`nalgebra::Vector2<f32>`].
pub type Vec2 = nalgebra::Vector2<f32>;

/// An alias to [`nalgebra::Vector3<f32>`].
pub type Vec3 = nalgebra::Vector3<f32>;

/// An alias to [`nalgebra::Vector4<f32>`].
pub type Vec4 = nalgebra::Vector4<f32>;

/// An alias to [`nalgebra::Matrix4<f32>`].
pub type Mat4 = nalgebra::Matrix4<f32>;

/// An alias to [`nalgebra::Orthographic3<f32>`].
pub type Ortho = nalgebra::Orthographic3<f32>;

/// An RGBA color represented with four [`f32`]s.
/// 
/// [`Color32`] is definied as a tuple-styled struct, with public members.
/// 
/// # Examples
/// 
/// ```
/// use moon::math::Color32;
/// let color = Color32::default();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Color32(pub f32, pub f32, pub f32, pub f32);

impl Default for Color32 {
    fn default() -> Self {
        Self(1.0, 1.0, 1.0, 1.0)
    }
}

impl Color32 {
    /// Get the Red component of the [`Color32`].
    /// 
    /// This is the same as the X component.
    pub const fn r(&self) -> f32 {
        self.0
    }
    /// Get the X component of the [`Color32`].
    /// 
    /// This is the same as the Red component.
    pub const fn x(&self) -> f32 {
        self.0
    }
    /// Get the Green component of the [`Color32`].
    /// 
    /// This is the same as the Y component.
    pub const fn g(&self) -> f32 {
        self.1
    }
    /// Get the Y component of the [`Color32`].
    /// 
    /// This is the same as the Green component.
    pub const fn y(&self) -> f32 {
        self.1
    }
    /// Get the Blue component of the [`Color32`].
    /// 
    /// This is the same as the Z component.
    pub const fn b(&self) -> f32 {
        self.2
    }
    /// Get the Z component of the [`Color32`].
    /// 
    /// This is the same as the Blue component.
    pub const fn z(&self) -> f32 {
        self.2
    }
    /// Get the Alpha component of the [`Color32`].
    /// 
    /// This is the same as the W component.
    pub const fn a(&self) -> f32 {
        self.3
    }
    /// Get the W component of the [`Color32`].
    /// 
    /// This is the same as the Alpha component.
    pub const fn w(&self) -> f32 {
        self.3
    }
}

impl From<&[f32; 4]> for Color32 {
    fn from(slice: &[f32; 4]) -> Self {
        Self(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<Color32> for [f32; 4] {
    fn from(color: Color32) -> Self {
        [color.0, color.1, color.2, color.3]
    }
}

impl From<&[u8; 4]> for Color32 {
    fn from(slice: &[u8; 4]) -> Self {
        Self::from(Color8(slice[0], slice[1], slice[2], slice[3]))
    }
}

impl From<Color32> for [u8; 4] {
    fn from(color: Color32) -> Self {
        let color = Color8::from(color);
        [color.0, color.1, color.2, color.3]
    }
}

impl From<Color32> for Vec<u8> {
    fn from(color: Color32) -> Self {
        let color = Color8::from(color);
        vec![color.0, color.1, color.2, color.3]
    }
}

impl From<Color8> for Color32 {
    fn from(color: Color8) -> Self {
        Self(
            color.0 as f32 / 255.0,
            color.1 as f32 / 255.0,
            color.2 as f32 / 255.0,
            color.3 as f32 / 255.0,
        )
    }
}

/// An RGBA color represented with four [`u8`]s.
/// 
/// [`Color8`] is definied as a tuple-styled struct, with public members.
/// 
/// # Examples
/// 
/// ```
/// use moon::math::Color8;
/// let color = Color8::default();
/// ```
#[derive(Clone, Copy)]
pub struct Color8(u8, u8, u8, u8);

impl Default for Color8 {
    fn default() -> Self {
        Self(255, 255, 255, 255)
    }
}

impl Color8 {
    /// Get the Red component of the [`Color8`].
    /// 
    /// This is the same as the X component.
    pub const fn r(&self) -> u8 {
        self.0
    }
    /// Get the X component of the [`Color8`].
    /// 
    /// This is the same as the Red component.
    pub const fn x(&self) -> u8 {
        self.0
    }
    /// Get the Green component of the [`Color8`].
    /// 
    /// This is the same as the Y component.
    pub const fn g(&self) -> u8 {
        self.1
    }
    /// Get the Y component of the [`Color8`].
    /// 
    /// This is the same as the Green component.
    pub const fn y(&self) -> u8 {
        self.1
    }
    /// Get the Blue component of the [`Color8`].
    /// 
    /// This is the same as the Z component.
    pub const fn b(&self) -> u8 {
        self.2
    }
    /// Get the Z component of the [`Color8`].
    /// 
    /// This is the same as the Blue component.
    pub const fn z(&self) -> u8 {
        self.2
    }
    /// Get the Alpha component of the [`Color8`].
    /// 
    /// This is the same as the W component.
    pub const fn a(&self) -> u8 {
        self.3
    }
    /// Get the W component of the [`Color8`].
    /// 
    /// This is the same as the Alpha component.
    pub const fn w(&self) -> u8 {
        self.3
    }
}

impl From<&[u8; 4]> for Color8 {
    fn from(slice: &[u8; 4]) -> Self {
        Self(slice[0], slice[1], slice[2], slice[3])
    }
}

impl From<Color8> for [u8; 4] {
    fn from(color: Color8) -> Self {
        [color.0, color.1, color.2, color.3]
    }
}

impl From<Color32> for Color8 {
    fn from(color: Color32) -> Self {
        Self(
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8,
            (color.3 * 255.0) as u8,
        )
    }
}

/// Pure White Color.
pub const WHITE: Color32 = Color32(1.0, 1.0, 1.0, 1.0);
/// Pure Black Color.
pub const BLACK: Color32 = Color32(0.0, 0.0, 0.0, 1.0);
/// Magenta Color.
pub const MAGENTA: Color32 = Color32(1.0, 0.0, 1.0, 1.0);

/// A [`Point`] is an alias to Vec2.
pub type Point = Vec2;

pub use nalgebra::clamp;
