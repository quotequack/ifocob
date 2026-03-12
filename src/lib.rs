pub mod decoders;
pub mod resolvers;

use image::DynamicImage;
use thiserror::Error;
use resolvers::*;

#[derive(Debug, Error)]
pub enum IfError {
    #[error("unexpected end of data")]
    UnexpectedEof,
    #[error("decode error")]
    DecodeError(#[from] image::ImageError),
    #[error("encode error")]
    EncodeError(),
    #[error("unknown file magic: {magic:?}")]
    UnknownMagic { magic: Vec<u8> },
}

pub fn decode(data: &[u8]) -> DynamicImage {
    let codec: CodecId = resolve_magic(data)
        .expect("Failed to recognise magic");
    let img = resolve_decode(codec, data)
        .expect("Failed to decode image");
    img
}

pub fn encode(img: &DynamicImage, codec: CodecId) -> Vec<u8> {
    let img: Vec<u8> = resolve_encode(codec, img).expect("Failed to encode");
    img
}