use crate::IfError;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use qoi::{encode_to_vec,decode_to_vec};

pub const MAGIC: &[u8] = b"qoif";

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    let (header, decoded) = decode_to_vec(&data)
      .expect("invalid qoi data");
    let img: RgbaImage = ImageBuffer::from_raw(header.width, header.height, decoded)
      .expect("buffer mismatch");
    Ok(DynamicImage::ImageRgba8(img))
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let width = img.width();
    let height = img.height();
    let data = ImageBuffer::into_raw(img.clone().into_rgba8());
    Ok(encode_to_vec(&data, width, height).expect("failed to encode qoi"))
}