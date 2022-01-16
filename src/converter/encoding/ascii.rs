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
///
/// # Examples
/// ```
/// use conversion::converter::encoding::ascii::{ASCIIDecoder, ASCIIEncodingError};
/// use conversion::iter::ConvertedIterator;
///
/// let iter = b"stra\xc3\x9fe".into_iter().cloned();
/// let mut decoded = ConvertedIterator::new(iter, ASCIIDecoder::new());
///
/// assert_eq!(Some(Ok('s')), decoded.next());
/// assert_eq!(Some(Ok('t')), decoded.next());
/// assert_eq!(Some(Ok('r')), decoded.next());
/// assert_eq!(Some(Ok('a')), decoded.next());
/// assert_eq!(Some(Err(ASCIIEncodingError)), decoded.next());
/// assert_eq!(Some(Err(ASCIIEncodingError)), decoded.next());
/// assert_eq!(Some(Ok('e')), decoded.next());
/// assert_eq!(None, decoded.next());
/// ```
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// An encoder for 7-bit ASCII.
///
/// # Examples
/// ```
/// use conversion::converter::encoding::ascii::{ASCIIEncoder, ASCIIEncodingError};
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "straße".chars();
/// let mut encoded = ConvertedIterator::new(iter, ASCIIEncoder::new());
///
/// assert_eq!(Some(Ok(b's')), encoded.next());
/// assert_eq!(Some(Ok(b't')), encoded.next());
/// assert_eq!(Some(Ok(b'r')), encoded.next());
/// assert_eq!(Some(Ok(b'a')), encoded.next());
/// assert_eq!(Some(Err(ASCIIEncodingError)), encoded.next());
/// assert_eq!(Some(Ok(b'e')), encoded.next());
/// assert_eq!(None, encoded.next());
/// ```
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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
