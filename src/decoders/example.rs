use crate::IfError;

pub const MAGIC: &[u8] = &[]; // Alt version
pub const MAGIC: &[u8] = b""; // Add the format's magic

pub fn decode(data: &[u8]) -> Result<DynamicImage, IfError> {
    // LOGIC
}

pub fn encode(img: &DynamicImage) -> Result<Vec<u8>, IfError> {
    // LOGIC
}