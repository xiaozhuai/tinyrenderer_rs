mod color;
mod fps;
mod framebuffer;
mod image_rw;
mod model;
mod primitive;
mod texture;

pub use color::Color;
pub use color::Colorf;
pub use fps::Fps;
pub use fps::FpsRet;
pub use framebuffer::Framebuffer;
pub use framebuffer::FramebufferError;
pub use image_rw::image_read;
pub use image_rw::image_write;
pub use image_rw::ImageReadError;
pub use image_rw::ImageWriteError;
pub use model::Model;
pub use primitive::draw_line;
pub use primitive::draw_triangle;
pub use texture::Texture2D;
pub use texture::Texture2DFilterMode;
pub use texture::Texture2DWrapMode;

pub type Vec2i = nalgebra::Vector2<i32>;
pub type Vec3i = nalgebra::Vector3<i32>;
pub type Vec4i = nalgebra::Vector4<i32>;

pub type Mat2x3 = nalgebra::Matrix2x3<f32>;
pub type Mat3 = nalgebra::Matrix3<f32>;

pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;
