//! UTF-32 Decoder/Encoder.

use crate::Converter;
use core::{convert::Infallible, fmt};

/// An error while encoding/decoding ASCII characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTF32EncodingError;

impl fmt::Display for UTF32EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a codepoint is out of bound.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UTF32EncodingError {}

/// A 32-bit decoder for UTF-32.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32Decoder;

impl UTF32Decoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32Decoder {
    type Item = u32;
    type Output = char;
    type Error = UTF32EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        match char::from_u32(item) {
            Some(c) => {
                buf.extend([c]);
                Ok(1)
            }
            None => Err(UTF32EncodingError),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// A 32-bit encoder for UTF32.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32Encoder;

impl UTF32Encoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32Encoder {
    type Item = char;
    type Output = u32;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend([item as u32]);
        Ok(1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
