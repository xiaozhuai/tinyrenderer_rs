use crate::Color;
use stb_image_rust::stbi_load_from_memory;
use stb_image_write_rust::ImageWriter::ImageWriter;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::slice;

#[derive(Debug)]
pub enum ImageReadError {
    IoError(std::io::Error),
    DecodeError,
}

impl From<std::io::Error> for ImageReadError {
    fn from(error: std::io::Error) -> Self {
        ImageReadError::IoError(error)
    }
}

#[derive(Debug)]
pub enum ImageWriteError {
    UnsupportedImageType,
}

pub fn image_read(
    filepath: impl AsRef<Path>,
    width: &mut i32,
    height: &mut i32,
) -> Result<Vec<Color>, ImageReadError> {
    let mut f = File::open(filepath)?;
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents)?;

    *width = 0;
    *height = 0;
    let mut comp = 0;
    let img: *mut u8;
    unsafe {
        img = stbi_load_from_memory(
            contents.as_ptr(),
            contents.len() as i32,
            width,
            height,
            &mut comp,
            4,
        );
    }

    if img.is_null() || *width == 0 || *height == 0 {
        return Err(ImageReadError::DecodeError);
    }

    let pixels_num = (*width * *height) as usize;
    let mut pixels: Vec<Color> = vec![Color::transparent(); pixels_num];
    let s = unsafe { slice::from_raw_parts(img as *const Color, pixels_num) };
    pixels.copy_from_slice(s);
    unsafe {
        stb_image_rust::c_runtime::free(img);
    }
    Ok(pixels)
}

pub fn image_write(
    filepath: impl AsRef<Path>,
    data: &[u8],
    width: i32,
    height: i32,
    comp: i32,
) -> Result<(), ImageWriteError> {
    if let Some(ext) = filepath.as_ref().extension() {
        let mut writer = ImageWriter::new(filepath.as_ref().to_str().unwrap());
        let ext = ext.to_ascii_lowercase();
        let ptr = data.as_ptr();
        if ext == "jpg" || ext == "jpeg" {
            writer.write_jpg(width, height, comp, ptr, 90);
            return Ok(());
        } else if ext == "png" {
            writer.write_png(width, height, comp, ptr);
            return Ok(());
        } else if ext == "bmp" {
            writer.write_bmp(width, height, comp, ptr);
            return Ok(());
        } else if ext == "tga" {
            writer.write_tga(width, height, comp, ptr);
            return Ok(());
        }
    }
    Err(ImageWriteError::UnsupportedImageType)
}
