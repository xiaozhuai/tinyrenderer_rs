use std::path::Path;
use std::slice;

use crate::{image_write, Color, ImageWriteError};

pub struct Framebuffer {
    color_buffer: Vec<Color>,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub enum FramebufferError {
    BadSize,
    BadPosition,
    ImageWriteError(ImageWriteError),
}

impl From<ImageWriteError> for FramebufferError {
    fn from(error: ImageWriteError) -> Self {
        FramebufferError::ImageWriteError(error)
    }
}

impl Framebuffer {
    pub fn create(width: i32, height: i32) -> Result<Self, FramebufferError> {
        Self::create_init_color(width, height, &Color::transparent())
    }

    pub fn create_init_color(
        width: i32,
        height: i32,
        color: &Color,
    ) -> Result<Self, FramebufferError> {
        if width < 0 || height < 0 {
            return Err(FramebufferError::BadSize);
        }
        Ok(Framebuffer {
            color_buffer: vec![*color; (width * height) as usize],
            width,
            height,
        })
    }

    fn calc_offset(&self, x: i32, y: i32) -> Result<usize, FramebufferError> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(FramebufferError::BadPosition);
        }
        Ok((y * self.width + x) as usize)
    }

    pub fn clear_color_with(&mut self, color: &Color) {
        self.color_buffer.fill(*color);
    }

    pub fn clear_color(&mut self) {
        self.clear_color_with(&Color::transparent());
    }

    pub fn set_color(&mut self, x: i32, y: i32, color: &Color) {
        if let Ok(offset) = self.calc_offset(x, y) {
            self.color_buffer[offset] = *color;
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> Result<&Color, FramebufferError> {
        let offset = self.calc_offset(x, y)?;
        Ok(&(self.color_buffer[offset]))
    }

    pub fn to_u8_ptr(&self) -> *const u8 {
        self.color_buffer.as_ptr() as *const u8
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
        self.color_buffer.as_ptr() as *const u32
    }

    pub fn to_u32_slice(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                self.to_u32_ptr(),
                self.width as usize * self.height as usize,
            )
        }
    }

    pub fn write(&self, filepath: impl AsRef<Path>) -> Result<(), FramebufferError> {
        image_write(filepath, self.to_u8_slice(), self.width, self.height, 4).map_err(|e| e.into())
    }
}
