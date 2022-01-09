//! UTF-32 Decoder/Encoder.

use crate::Converter;
use core::{convert::Infallible, fmt};

/// An error while encoding/decoding UTF-32 characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTF32EncodingError;

impl fmt::Display for UTF32EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "found invalid UTF32 sequence.")
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// A 32-bit encoder for UTF-32.
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// A byte decoder for UTF-32 (big-endian).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32BEDecoder {
    bytes: [u8; 4],
    count: usize,
}

impl UTF32BEDecoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32BEDecoder {
    type Item = u8;
    type Output = char;
    type Error = UTF32EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        self.bytes[self.count] = item;
        if self.count == 3 {
            self.count = 0;
            UTF32Decoder.convert(u32::from_be_bytes(self.bytes), buf)
        } else {
            self.count += 1;
            Ok(0)
        }
    }

    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        if self.count == 0 {
            Ok(())
        } else {
            Err(UTF32EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// A byte encoder for UTF-32 (big-endian).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32BEEncoder;

impl UTF32BEEncoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32BEEncoder {
    type Item = char;
    type Output = u8;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend((item as u32).to_be_bytes());
        Ok(4)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (4, Some(4))
    }
}

/// A byte decoder for UTF-32 (little-endian).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32LEDecoder {
    bytes: [u8; 4],
    count: usize,
}

impl UTF32LEDecoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32LEDecoder {
    type Item = u8;
    type Output = char;
    type Error = UTF32EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        self.bytes[self.count] = item;
        if self.count == 3 {
            self.count = 0;
            UTF32Decoder.convert(u32::from_le_bytes(self.bytes), buf)
        } else {
            self.count += 1;
            Ok(0)
        }
    }

    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        if self.count == 0 {
            Ok(())
        } else {
            Err(UTF32EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// A byte encoder for UTF-32 (little-endian).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF32LEEncoder;

impl UTF32LEEncoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF32LEEncoder {
    type Item = char;
    type Output = u8;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend((item as u32).to_le_bytes());
        Ok(4)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (4, Some(4))
    }
}
