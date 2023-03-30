use crate::{image_read, image_write, Color, Colorf, ImageReadError, ImageWriteError};
use std::path::Path;
use std::slice;

pub struct Texture2D {
    pixels: Vec<Color>,
    border_color: Color,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub enum Texture2DError {
    BadSize,
    BadPosition,
    ImageReadError(ImageReadError),
    ImageWriteError(ImageWriteError),
}

impl From<ImageReadError> for Texture2DError {
    fn from(error: ImageReadError) -> Self {
        Texture2DError::ImageReadError(error)
    }
}

impl From<ImageWriteError> for Texture2DError {
    fn from(error: ImageWriteError) -> Self {
        Texture2DError::ImageWriteError(error)
    }
}

#[derive(Copy, Clone)]
pub enum Texture2DWrapMode {
    ClampToEdge,
    ClampToBorder,
    Repeat,
    MirroredRepeat,
}

#[derive(Copy, Clone)]
pub enum Texture2DFilterMode {
    Nearest,
    Linear,
}

impl Texture2D {
    pub fn create(width: i32, height: i32) -> Result<Self, Texture2DError> {
        Self::create_init_color(width, height, &Color::transparent())
    }

    pub fn create_init_color(
        width: i32,
        height: i32,
        color: &Color,
    ) -> Result<Self, Texture2DError> {
        if width < 0 || height < 0 {
            return Err(Texture2DError::BadSize);
        }
        Ok(Texture2D {
            pixels: vec![*color; (width * height) as usize],
            border_color: Color::transparent(),
            width,
            height,
        })
    }

    pub fn load(filepath: impl AsRef<Path>) -> Result<Self, Texture2DError> {
        let mut width = 0;
        let mut height = 0;
        let pixels = image_read(filepath, &mut width, &mut height)?;
        Ok(Texture2D {
            pixels,
            border_color: Color::transparent(),
            width,
            height,
        })
    }

    pub fn to_u8_ptr(&self) -> *const u8 {
        self.pixels.as_ptr() as *const u8
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.to_u8_ptr(),
                self.width as usize * self.height as usize * std::mem::size_of::<Color>(),
            )
        }
    }

    pub fn to_u32_ptr(&self) -> *const u32 {
        self.pixels.as_ptr() as *const u32
    }

    pub fn to_u32_slice(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                self.to_u32_ptr(),
                self.width as usize * self.height as usize,
            )
        }
    }

    pub fn write(&self, filepath: impl AsRef<Path>) -> Result<(), Texture2DError> {
        image_write(filepath, self.to_u8_slice(), self.width, self.height, 4).map_err(|e| e.into())
    }

    fn calc_offset(&self, x: i32, y: i32) -> Result<usize, Texture2DError> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(Texture2DError::BadPosition);
        }
        Ok((y * self.width + x) as usize)
    }

    pub fn get_color(&self, x: i32, y: i32) -> Result<&Color, Texture2DError> {
        let offset = self.calc_offset(x, y)?;
        Ok(&(self.pixels[offset]))
    }

    pub fn set_border_color(&mut self, color: &Color) {
        self.border_color = *color;
    }

    fn my_mod<T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>(n: T, m: T) -> T {
        ((n % m) + m) % m
    }

    fn wrap_coord(x: f32, y: f32, wrap_mode: Texture2DWrapMode) -> (f32, f32, bool) {
        let use_border_color = !(0f32..=1f32).contains(&x) || !(0f32..=1f32).contains(&y);
        match wrap_mode {
            Texture2DWrapMode::ClampToEdge => (x.clamp(0f32, 1f32), y.clamp(0f32, 1f32), false),
            Texture2DWrapMode::ClampToBorder => {
                (x.clamp(0f32, 1f32), y.clamp(0f32, 1f32), use_border_color)
            }
            Texture2DWrapMode::Repeat => (Self::my_mod(x, 1f32), Self::my_mod(y, 1f32), false),
            Texture2DWrapMode::MirroredRepeat => (
                Self::my_mod((1f32 - Self::my_mod(x.floor(), 2f32) * 2f32) * x, 1f32),
                Self::my_mod((1f32 - Self::my_mod(y.floor(), 2f32) * 2f32) * y, 1f32),
                false,
            ),
        }
    }

    pub fn texture(
        &self,
        x: f32,
        y: f32,
        wrap_mode: Texture2DWrapMode,
        filter_mode: Texture2DFilterMode,
    ) -> Colorf {
        let (x, y, use_border_color) = Self::wrap_coord(x, 1f32 - y, wrap_mode);
        if use_border_color {
            return self.border_color.into();
        }
        match filter_mode {
            Texture2DFilterMode::Nearest => {
                let x = (x * self.width as f32).round() as i32;
                let y = (y * self.height as f32).round() as i32;
                (*self.get_color(x, y).unwrap()).into()
            }
            Texture2DFilterMode::Linear => {
                let x_min = (x * self.width as f32).floor() as i32;
                let y_min = (y * self.height as f32).floor() as i32;
                let x_max = (x * self.width as f32).ceil() as i32;
                let y_max = (y * self.height as f32).ceil() as i32;
                let x = x * self.width as f32;
                let y = y * self.height as f32;
                let xt = if x_min == x_max {
                    0f32
                } else {
                    (x - x_min as f32) / (x_max - x_min) as f32
                };
                let yt = if y_min == y_max {
                    0f32
                } else {
                    (y - y_min as f32) / (y_max - y_min) as f32
                };
                let color1: Colorf = (*(self.get_color(x_min, y_min).unwrap())).into();
                let color2: Colorf = (*(self.get_color(x_max, y_min).unwrap())).into();
                let color3: Colorf = (*(self.get_color(x_min, y_max).unwrap())).into();
                let color4: Colorf = (*(self.get_color(x_max, y_max).unwrap())).into();
                color1 * (1f32 - xt) * (1f32 - yt)
                    + color2 * xt * (1f32 - yt)
                    + color3 * (1f32 - xt) * yt
                    + color4 * xt * yt
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    macro_rules! assert_wrap_coord_eq {
        ($x:expr, $y:expr) => {
            assert_relative_eq!($x.0, $y.0);
            assert_relative_eq!($x.1, $y.1);
            assert_eq!($x.2, $y.2);
        };
    }

    #[test]
    fn test_wrap_coord() {
        assert_wrap_coord_eq!(
            (0.5f32, 0.5f32, false),
            Texture2D::wrap_coord(0.5f32, 0.5f32, Texture2DWrapMode::ClampToEdge)
        );
        assert_wrap_coord_eq!(
            (1f32, 0f32, false),
            Texture2D::wrap_coord(1.5f32, -0.5f32, Texture2DWrapMode::ClampToEdge)
        );
        assert_wrap_coord_eq!(
            (0.5f32, 0.5f32, false),
            Texture2D::wrap_coord(0.5f32, 0.5f32, Texture2DWrapMode::ClampToBorder)
        );
        assert_wrap_coord_eq!(
            (1f32, 0f32, true),
            Texture2D::wrap_coord(1.5f32, -0.5f32, Texture2DWrapMode::ClampToBorder)
        );
        assert_wrap_coord_eq!(
            (0.2f32, 0.6f32, false),
            Texture2D::wrap_coord(1.2f32, -0.4f32, Texture2DWrapMode::Repeat)
        );
        assert_wrap_coord_eq!(
            (0.2f32, 0.4f32, false),
            Texture2D::wrap_coord(1.2f32, -0.6f32, Texture2DWrapMode::Repeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.4f32, false),
            Texture2D::wrap_coord(1.2f32, -0.4f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.6f32, false),
            Texture2D::wrap_coord(1.2f32, -0.6f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.6f32, false),
            Texture2D::wrap_coord(1.2f32, -1.4f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.4f32, false),
            Texture2D::wrap_coord(1.2f32, -1.6f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.4f32, false),
            Texture2D::wrap_coord(1.2f32, 0.4f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.6f32, false),
            Texture2D::wrap_coord(1.2f32, 0.6f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.6f32, false),
            Texture2D::wrap_coord(1.2f32, 1.4f32, Texture2DWrapMode::MirroredRepeat)
        );
        assert_wrap_coord_eq!(
            (0.8f32, 0.4f32, false),
            Texture2D::wrap_coord(1.2f32, 1.6f32, Texture2DWrapMode::MirroredRepeat)
        );
    }
}
