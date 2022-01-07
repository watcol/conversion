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
        let tmp_buf = [0u8; 4];
        let res = item.encode_utf8(&mut tmp_buf);
        buf.extend(tmp_buf);
        Ok(res.len())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(4))
    }
}
