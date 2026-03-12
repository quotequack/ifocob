use image::{DynamicImage, ImageFormat};
use crate::IfError;

pub const MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    image::load_from_memory_with_format(data, ImageFormat::Png)
        .map_err(IfError::DecodeError)
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let _ = img.write_to(&mut buf, ImageFormat::Png);
    Ok(buf.into_inner())
}