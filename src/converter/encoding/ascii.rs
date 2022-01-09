//! 7-bit ASCII Decoder/Encoder.

use crate::Converter;
use core::fmt;

/// An error while encoding/decoding ASCII characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ASCIIEncodingError;

impl fmt::Display for ASCIIEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A character out of bound.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ASCIIEncodingError {}

/// A decoder for 7-bit ASCII.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ASCIIDecoder;

impl ASCIIDecoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for ASCIIDecoder {
    type Item = u8;
    type Output = char;
    type Error = ASCIIEncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        if item <= 0x7f {
            buf.extend([item as char]);
            Ok(1)
        } else {
            Err(ASCIIEncodingError)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// An encoder for 7-bit ASCII.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ASCIIEncoder;

impl ASCIIEncoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for ASCIIEncoder {
    type Item = char;
    type Output = u8;
    type Error = ASCIIEncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        if (item as u32) <= 0x7f {
            buf.extend([item as u8]);
            Ok(1)
        } else {
            Err(ASCIIEncodingError)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
