//! This crate provides encoding for ICNS icons (Apple Icon Image Format).

mod decode;
mod encode;
mod os_type;

pub use crate::decode::Decoder;
pub use crate::encode::Encoder;
