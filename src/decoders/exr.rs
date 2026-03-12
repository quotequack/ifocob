use image::{DynamicImage, ImageFormat};
use crate::IfError;

pub const MAGIC: &[u8] = &[0x76, 0x2F, 0x31, 0x01];

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    image::load_from_memory_with_format(data, ImageFormat::OpenExr)
        .map_err(IfError::DecodeError)
        .map(|img| DynamicImage::ImageRgb8(img.into_rgb8()))
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let img = image::DynamicImage::ImageRgb32F(img.clone().into_rgb32f());
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::OpenExr)
        .map_err(IfError::EncodeError)?;
    Ok(buf.into_inner())
}