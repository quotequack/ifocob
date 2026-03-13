use image::{DynamicImage, ImageFormat};
use crate::IfError;

pub const MAGIC: &[u8] = &[0x49, 0x49, 0x2A, 0x00];

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    image::load_from_memory_with_format(data, ImageFormat::Tiff)
        .map_err(IfError::DecodeError)
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let _ = img.write_to(&mut buf, ImageFormat::Tiff);
    Ok(buf.into_inner())
}