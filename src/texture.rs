use crate::{image_read, image_write, Color, Colorf, ImageReadError, ImageWriteError};
use std::path::Path;
use std::slice;

pub struct Texture2D {
    pixels: Vec<Color>,
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

    pub fn texture(&self, x: f32, y: f32) -> Colorf {
        let y = 1f32 - y;
        let x = x.clamp(0f32, 1f32);
        let y = y.clamp(0f32, 1f32);

        let x = (x * self.width as f32).round() as i32;
        let y = (y * self.height as f32).round() as i32;
        if let Ok(color) = self.get_color(x, y) {
            (*color).into()
        } else {
            Color::transparent().into()
        }
    }
}
