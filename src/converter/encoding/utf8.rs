//! UTF-8 Decoder/Encoder.

use crate::Converter;
use core::convert::Infallible;
use core::fmt;

/// An error while decoding UTF-8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTF8EncodingError;

impl fmt::Display for UTF8EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "found invalid UTF-8 sequence.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UTF8EncodingError {}

/// A decoder for UTF-8
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF8Decoder {
    // remaining bytes to construct one character.
    remain: u8,
    // current UTF8 codepoint.
    codepoint: u32,
    // lower bound of the second, third or fourth byte.
    lower: u8,
}

impl UTF8Decoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF8Decoder {
    type Item = u8;
    type Output = char;
    type Error = UTF8EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        if self.remain == 0 {
            // first byte
            match item {
                0x00..=0x7F => {
                    buf.extend([item as char]);
                    Ok(1)
                }
                0xC2..=0xDF => {
                    self.remain = 1;
                    self.codepoint = ((item & 0b0001_1111) as u32) << 6;
                    self.lower = 0x80;
                    Ok(0)
                }
                0xE0..=0xEF => {
                    self.remain = 2;
                    self.codepoint = ((item & 0b0000_1111) as u32) << 12;
                    self.lower = 0xA0;
                    Ok(0)
                }
                0xF0..=0xF4 => {
                    self.remain = 3;
                    self.codepoint = ((item & 0b0000_0111) as u32) << 18;
                    self.lower = 0x90;
                    Ok(0)
                }
                _ => Err(UTF8EncodingError),
            }
        } else {
            self.remain -= 1;
            if (self.lower..0xBF).contains(&item) {
                self.codepoint |= ((item & 0b0011_1111) as u32) << (self.remain * 6);
            } else {
                return Err(UTF8EncodingError);
            }

            if self.remain == 0 {
                buf.extend([unsafe { char::from_u32_unchecked(self.codepoint) }]);
                Ok(1)
            } else {
                self.lower = 0x80;
                Ok(0)
            }
        }
    }

    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        if self.remain == 0 {
            Ok(())
        } else {
            Err(UTF8EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// An encoder for UTF-8.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF8Encoder;

impl UTF8Encoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF8Encoder {
    type Item = char;
    type Output = u8;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        let mut tmp_buf = [0u8; 4];
        let len = item.encode_utf8(&mut tmp_buf).len();
        buf.extend(tmp_buf.into_iter().take(len));
        Ok(len)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(4))
    }
}
