use std::path::Path;
use std::slice;

use crate::{image_write, Color, ImageWriteError};

pub struct Framebuffer {
    color_buffer: Vec<Color>,
    depth_buffer: Vec<f32>,
    depth_test: bool,
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
            depth_buffer: vec![f32::MIN; (width * height) as usize],
            depth_test: true,
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

    pub fn clear_depth_with(&mut self, depth: f32) {
        self.depth_buffer.fill(depth);
    }

    pub fn clear_depth(&mut self) {
        self.clear_depth_with(f32::MIN);
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

    pub fn set_depth_test(&mut self, enable: bool) {
        self.depth_test = enable;
    }

    pub fn set_depth(&mut self, x: i32, y: i32, depth: f32) {
        if let Ok(offset) = self.calc_offset(x, y) {
            self.depth_buffer[offset] = depth;
        }
    }

    pub fn get_depth(&self, x: i32, y: i32) -> f32 {
        if let Ok(offset) = self.calc_offset(x, y) {
            self.depth_buffer[offset]
        } else {
            f32::MAX
        }
    }

    pub fn set_color_with_depth(&mut self, x: i32, y: i32, depth: f32, color: &Color) {
        if self.depth_test {
            if ((-1f32 - f32::EPSILON)..=(1f32 + f32::EPSILON)).contains(&depth)
                && depth > self.get_depth(x, y)
            {
                self.set_color(x, y, color);
                self.set_depth(x, y, depth);
            }
        } else {
            self.set_color(x, y, color);
        }
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

    pub fn write_depth(&self, filepath: impl AsRef<Path>) -> Result<(), FramebufferError> {
        let mut depth_buffer: Vec<u8> = vec![0; self.depth_buffer.len()];
        for i in 0..self.depth_buffer.len() {
            depth_buffer[i] = ((self.depth_buffer[i] / 2f32 + 0.5f32) * 255f32).round() as u8;
        }
        image_write(
            filepath,
            depth_buffer.as_slice(),
            self.width,
            self.height,
            1,
        )
        .map_err(|e| e.into())
    }
}
