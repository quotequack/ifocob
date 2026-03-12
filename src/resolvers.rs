use crate::{IfError, decoders};
use image::DynamicImage;

pub fn resolve_decode(codec: CodecId, payload: &[u8]) -> Result<image::DynamicImage, IfError> {
    let out: Result<image::DynamicImage, IfError> = match codec {
        // Add your codec here
        CodecId::Png  => decoders::png::decode(payload),
        CodecId::Jpeg => decoders::jpeg::decode(payload),
        CodecId::Bmp  => decoders::bmp::decode(payload),
        CodecId::Qoi => decoders::qoi::decode(payload),
        CodecId::Exr => decoders::exr::decode(payload),
    };
    out
}

pub fn resolve_encode(codec: CodecId, payload: &DynamicImage) -> Result<Vec<u8>, IfError> {
    let out: Result<Vec<u8>, IfError> = match codec {
        // Add your codec here
        CodecId::Png  => decoders::png::encode(payload),
        CodecId::Jpeg => decoders::jpeg::encode(payload),
        CodecId::Bmp  => decoders::bmp::encode(payload),
        CodecId::Qoi => decoders::qoi::encode(payload),
        CodecId::Exr => decoders::exr::encode(payload),
    };
    out
}

pub fn resolve_name(codec: String) -> CodecId {
    let codec: CodecId = match codec.to_lowercase().as_str() {
        "png"  => CodecId::Png,
        "jpeg" | "jpg" => CodecId::Jpeg,
        "bmp" | "bitmap"  => CodecId::Bmp,
        "qoi" => CodecId::Qoi,
        "exr" | "openexr" => CodecId::Exr,
        other  => panic!("unknown codec: {}", other),
    };
    codec
}

fn matches(data: &[u8], magic: &[u8]) -> bool {
    data.starts_with(magic)
}

pub fn resolve_magic(data: &[u8]) -> Result<CodecId, IfError> {
    match () {
        _ if matches(data, decoders::png::MAGIC)  => Ok(CodecId::Png),
        _ if matches(data, decoders::jpeg::MAGIC) => Ok(CodecId::Jpeg),
        _ if matches(data, decoders::bmp::MAGIC)  => Ok(CodecId::Bmp),
        _ if matches(data, decoders::qoi::MAGIC)  => Ok(CodecId::Qoi),
        _ if matches(data, decoders::exr::MAGIC)  => Ok(CodecId::Exr),
        _ => Err(IfError::UnknownMagic { magic: data[..8.min(data.len())].to_vec() }),
    }
}

pub enum CodecId {
    Png,
    Jpeg,
    Bmp,
    Qoi,
    Exr,
}