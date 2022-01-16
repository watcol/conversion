//! UTF-16 Decoder/Encoder.

use crate::Converter;
use core::convert::Infallible;
use core::fmt;

use super::utf32::UTF32Decoder;

/// An error while encoding/decoding UTF-32 characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UTF16EncodingError;

impl fmt::Display for UTF16EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "found invalid UTF16 sequence.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UTF16EncodingError {}

/// A 16-bit decoder for UTF-16.
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::{UTF16Decoder, UTF16EncodingError};
/// use conversion::iter::ConvertedIterator;
///
/// let iter = [
///     0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834
/// ].into_iter();
/// let mut decoded = ConvertedIterator::new(iter, UTF16Decoder::new());
///
/// assert_eq!(Some(Ok('𝄞')), decoded.next());
/// assert_eq!(Some(Ok('m')), decoded.next());
/// assert_eq!(Some(Ok('u')), decoded.next());
/// assert_eq!(Some(Ok('s')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// assert_eq!(Some(Ok('i')), decoded.next());
/// assert_eq!(Some(Ok('c')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16Decoder {
    buf: Option<u16>,
}

impl UTF16Decoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16Decoder {
    type Item = u16;
    type Output = char;
    type Error = UTF16EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        match self.buf {
            Some(w) if item & 0xFC00 == 0xDC00 => {
                self.buf = None;
                UTF32Decoder
                    .convert(
                        ((((w & 0x3C0) as u32) >> 6) + 1) << 16
                            | ((w & 0x3F) as u32) << 10
                            | (item & 0x3FF) as u32,
                        buf,
                    )
                    .map_err(|_| UTF16EncodingError)
            }
            Some(_) => {
                self.buf = None;
                Err(UTF16EncodingError)
            }
            None if item & 0xFC00 == 0xD800 => {
                self.buf = Some(item);
                Ok(0)
            }
            None if item & 0xFC00 == 0xDC00 => Err(UTF16EncodingError),
            None => UTF32Decoder
                .convert(item as u32, buf)
                .map_err(|_| UTF16EncodingError),
        }
    }

    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        if self.buf.is_none() {
            Ok(())
        } else {
            Err(UTF16EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// A 16-bit encoder for UTF-16.
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::UTF16Encoder;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "𝄞music".chars();
/// let decoded = ConvertedIterator::new(iter, UTF16Encoder::new());
///
/// assert_eq!(Ok(vec![
///     0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063,
/// ]), decoded.collect());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16Encoder;

impl UTF16Encoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16Encoder {
    type Item = char;
    type Output = u16;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        let mut tmp_buf = [0u16; 2];
        let len = item.encode_utf16(&mut tmp_buf).len();
        buf.extend(tmp_buf.into_iter().take(len));
        Ok(len)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(2))
    }
}

/// A byte decoder for UTF-16 (big-endian).
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::{UTF16BEDecoder, UTF16EncodingError};
/// use conversion::iter::ConvertedIterator;
///
/// let iter = b"\xD8\x34\xDD\x1E\x00\x6d\x00\x75\x00\x73\xDD\x1E\x00\x69\x00\x63\xD8\x34"
///     .into_iter().cloned();
/// let mut decoded = ConvertedIterator::new(iter, UTF16BEDecoder::new());
///
/// assert_eq!(Some(Ok('𝄞')), decoded.next());
/// assert_eq!(Some(Ok('m')), decoded.next());
/// assert_eq!(Some(Ok('u')), decoded.next());
/// assert_eq!(Some(Ok('s')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// assert_eq!(Some(Ok('i')), decoded.next());
/// assert_eq!(Some(Ok('c')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16BEDecoder {
    byte: Option<u8>,
    inner: UTF16Decoder,
}

impl UTF16BEDecoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16BEDecoder {
    type Item = u8;
    type Output = char;
    type Error = UTF16EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        match self.byte {
            Some(b) => {
                self.byte = None;
                self.inner.convert(u16::from_be_bytes([b, item]), buf)
            }
            None => {
                self.byte = Some(item);
                Ok(0)
            }
        }
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        self.inner.finalize()?;

        if self.byte.is_some() {
            Ok(())
        } else {
            Err(UTF16EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// A byte encoder for UTF-16 (big-endian).
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::UTF16BEEncoder;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "𝄞music".chars();
/// let decoded = ConvertedIterator::new(iter, UTF16BEEncoder::new());
///
/// assert_eq!(Ok(b"\xD8\x34\xDD\x1E\x00\x6d\x00\x75\x00\x73\x00\x69\x00\x63".to_vec()), decoded.collect());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16BEEncoder;

impl UTF16BEEncoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16BEEncoder {
    type Item = char;
    type Output = u8;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        let mut tmp_buf = [0u16; 2];
        let len = item.encode_utf16(&mut tmp_buf).len();
        for w in tmp_buf.into_iter().take(len) {
            buf.extend(w.to_be_bytes());
        }
        Ok(len * 2)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (2, Some(4))
    }
}

/// A byte decoder for UTF-16 (little-endian).
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::{UTF16LEDecoder, UTF16EncodingError};
/// use conversion::iter::ConvertedIterator;
///
/// let iter = b"\x34\xD8\x1E\xDD\x6d\x00\x75\x00\x73\x00\x1E\xDD\x69\x00\x63\x00\x34\xD8"
///     .into_iter().cloned();
/// let mut decoded = ConvertedIterator::new(iter, UTF16LEDecoder::new());
///
/// assert_eq!(Some(Ok('𝄞')), decoded.next());
/// assert_eq!(Some(Ok('m')), decoded.next());
/// assert_eq!(Some(Ok('u')), decoded.next());
/// assert_eq!(Some(Ok('s')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// assert_eq!(Some(Ok('i')), decoded.next());
/// assert_eq!(Some(Ok('c')), decoded.next());
/// assert_eq!(Some(Err(UTF16EncodingError)), decoded.next());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16LEDecoder {
    byte: Option<u8>,
    inner: UTF16Decoder,
}

impl UTF16LEDecoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16LEDecoder {
    type Item = u8;
    type Output = char;
    type Error = UTF16EncodingError;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        match self.byte {
            Some(b) => {
                self.byte = None;
                self.inner.convert(u16::from_le_bytes([b, item]), buf)
            }
            None => {
                self.byte = Some(item);
                Ok(0)
            }
        }
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        self.inner.finalize()?;

        if self.byte.is_none() {
            Ok(())
        } else {
            Err(UTF16EncodingError)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(1))
    }
}

/// A byte encoder for UTF-16 (little-endian).
///
/// # Examples
/// ```
/// use conversion::converter::encoding::utf16::UTF16LEEncoder;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "𝄞music".chars();
/// let decoded = ConvertedIterator::new(iter, UTF16LEEncoder::new());
///
/// assert_eq!(Ok(b"\x34\xD8\x1E\xDD\x6d\x00\x75\x00\x73\x00\x69\x00\x63\x00".to_vec()), decoded.collect());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UTF16LEEncoder;

impl UTF16LEEncoder {
    /// Create a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Converter for UTF16LEEncoder {
    type Item = char;
    type Output = u8;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        let mut tmp_buf = [0u16; 2];
        let len = item.encode_utf16(&mut tmp_buf).len();
        for w in tmp_buf.into_iter().take(len) {
            buf.extend(w.to_le_bytes());
        }
        Ok(len * 2)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (2, Some(4))
    }
}
