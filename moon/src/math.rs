pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;

pub type Mat4 = nalgebra::Matrix4<f32>;
pub type Ortho = nalgebra::Orthographic3<f32>;

#[derive(Clone, Copy)]
pub struct Color32(f32, f32, f32, f32);

#[derive(Clone, Copy)]
pub struct Color8(u8, u8, u8, u8);

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

impl From<Color32> for Vec<u8> {
    fn from(color: Color32) -> Self {
        let color = Color8::from(color);
        vec![color.0, color.1, color.2, color.3]
    }
}

impl Color32 {
    pub const fn r(&self) -> f32 {
        self.0
    }
    pub const fn x(&self) -> f32 {
        self.0
    }
    pub const fn g(&self) -> f32 {
        self.1
    }
    pub const fn y(&self) -> f32 {
        self.1
    }
    pub const fn b(&self) -> f32 {
        self.2
    }
    pub const fn z(&self) -> f32 {
        self.2
    }
    pub const fn a(&self) -> f32 {
        self.3
    }
    pub const fn w(&self) -> f32 {
        self.3
    }
}

impl Color8 {
    pub const fn r(&self) -> u8 {
        self.0
    }
    pub const fn x(&self) -> u8 {
        self.0
    }
    pub const fn g(&self) -> u8 {
        self.1
    }
    pub const fn y(&self) -> u8 {
        self.1
    }
    pub const fn b(&self) -> u8 {
        self.2
    }
    pub const fn z(&self) -> u8 {
        self.2
    }
    pub const fn a(&self) -> u8 {
        self.3
    }
    pub const fn w(&self) -> u8 {
        self.3
    }
}

pub const WHITE: Color32 = Color32(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Color32 = Color32(0.0, 0.0, 0.0, 1.0);
pub const MAGENTA: Color32 = Color32(1.0, 0.0, 1.0, 1.0);

pub type Point = Vec2;

pub use nalgebra::clamp;
