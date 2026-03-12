use image::{DynamicImage, ImageFormat};
use crate::IfError;

pub const MAGIC: &[u8] = &[0x42, 0x4D];

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    image::load_from_memory_with_format(data, ImageFormat::Bmp)
        .map_err(IfError::DecodeError)
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Bmp)
        .map_err(IfError::DecodeError)?;
    Ok(buf.into_inner())
}