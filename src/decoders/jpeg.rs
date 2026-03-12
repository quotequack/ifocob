use image::{DynamicImage, ImageFormat};
use crate::IfError;

pub const MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    image::load_from_memory_with_format(data, ImageFormat::Jpeg)
        .map_err(IfError::DecodeError)
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Jpeg)
        .map_err(IfError::DecodeError)?;
    Ok(buf.into_inner())
}