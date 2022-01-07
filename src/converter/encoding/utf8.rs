//! UTF-8 Decoder/Encoder.

use core::convert::Infallible;

use crate::Converter;

/// An encoder for UTF-8.
pub struct UTF8Encoder;

impl Default for UTF8Encoder {
    #[inline]
    fn default() -> Self {
        Self
    }
}

impl UTF8Encoder {
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
        let code = item as u32;
        let len = match code {
            0x0000..=0x007F => {
                buf.extend([code as u8]);
                1
            }
            0x0080..=0x07FF => {
                buf.extend([
                    0b1100_0000 | ((code & 0b111_1100_0000) >> 6) as u8,
                    0b10000000 | ((code & 0b000_0011_1111) as u8),
                ]);
                2
            }
            0x0800..=0xFFFF => {
                buf.extend([
                    0b1110_0000 | ((code & 0b1111_0000_0000_0000) >> 12) as u8,
                    0b1000_0000 | ((code & 0b0000_1111_1100_0000) >> 6) as u8,
                    0b1000_0000 | ((code & 0b0000_0000_0011_1111) as u8),
                ]);
                3
            }
            0x10000..=0x10FFFF => {
                buf.extend([
                    0b1111_0000 | ((code & 0b1_1100_0000_0000_0000_0000) >> 18) as u8,
                    0b1000_0000 | ((code & 0b0_0011_1111_0000_0000_0000) >> 12) as u8,
                    0b1000_0000 | ((code & 0b0_0000_0000_1111_1100_0000) >> 6) as u8,
                    0b1000_0000 | ((code & 0b0_0000_0000_0000_0011_1111) as u8),
                ]);
                4
            }
            _ => unreachable!(),
        };
        Ok(len)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(4))
    }
}
