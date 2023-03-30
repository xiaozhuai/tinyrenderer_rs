use crate::Vec4;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Colorf = Vec4;

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn red() -> Self {
        Color::new(255, 0, 0, 255)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0, 255)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255, 255)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0, 255)
    }

    pub fn transparent() -> Self {
        Color::new(0, 0, 0, 0)
    }
}

impl From<Color> for Colorf {
    fn from(color: Color) -> Self {
        Colorf::new(
            color.r as f32 / 255f32,
            color.g as f32 / 255f32,
            color.b as f32 / 255f32,
            color.a as f32 / 255f32,
        )
    }
}

impl From<Colorf> for Color {
    fn from(color: Colorf) -> Self {
        Color::new(
            (color.x.clamp(0f32, 1f32) * 255f32).round() as u8,
            (color.y.clamp(0f32, 1f32) * 255f32).round() as u8,
            (color.z.clamp(0f32, 1f32) * 255f32).round() as u8,
            (color.w.clamp(0f32, 1f32) * 255f32).round() as u8,
        )
    }
}
