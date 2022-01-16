//! An abstraction crate to convert iterators on the fly.
//!
//! # Demo
//! ```
//! use conversion::converter::encoding::utf8::{UTF8Decoder, UTF8Encoder};
//! use conversion::converter::IterConverter;
//! use conversion::iter::{ConvertedIterator, ConvertedTryIterator};
//!
//! // An original byte string.
//! let iter = b"stra\xc3\x9fe".into_iter().cloned();
//!
//! // Decoding UTF-8 byte string.
//! let decoded = ConvertedIterator::new(iter, UTF8Decoder::new());
//! assert_eq!(Ok(String::from("straße")), decoded.clone().collect());
//!
//! // Convert to uppercase. (use ConvertedTryIterator because `decoded` returns Result items.)
//! let uppered = ConvertedTryIterator::new(decoded, IterConverter::new(char::to_uppercase));
//! assert_eq!(Ok(String::from("STRASSE")), uppered.clone().collect());
//!
//! // Re-encode the value.
//! let encoded = ConvertedTryIterator::new(uppered, UTF8Encoder::new());
//! assert_eq!(Ok(b"STRASSE".to_vec()), encoded.collect());
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod infallible;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod iter;
#[cfg(feature = "async")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "async")))]
pub mod stream;

pub mod converter;

use converter::ChainedConverter;

/// A trait for converters which converts N items into M outputs.
pub trait Converter {
    /// The type of input items.
    type Item;

    /// The type of outputs.
    type Output;

    /// The type of errors while conversion.
    type Error;

    /// Consumes one item, stores outputs into `buf` implements [`Extend`], and returns
    /// the number of stored outputs or a conversion error. If there are no outputs yet,
    /// you should store the item and return `Ok(0)`.
    ///
    /// [`Extend`]: core::iter::Extend
    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>;

    /// Returns whether the converter reached the end, or not.
    ///
    /// The default return value is `false`, and turning it `true` means the converter will no
    /// longer produce any outputs. (In other words, the [`convert`] method returns `Ok(0)`
    /// forever.)
    ///
    /// [`convert`]: Self::convert
    #[inline]
    fn is_ended(&self) -> bool {
        false
    }

    /// Finalizing the converter.
    ///
    /// This method will be called when input iterators or streams reached end (or the converter
    /// indicated the end by [`is_ended`] method.). You should finalize the converter or report
    /// remaining inputs which should be consumed inside them in this method. The default behavior
    /// is just returning `Ok(())`.
    ///
    /// [`is_ended`]: Self::is_ended
    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Returnd the estimated bounds about the numbers of outputs that one item will produce.
    ///
    /// The first element is the lower bound, and the second element is the upper bound. (if
    /// nothing, you can specify [`None`].)
    ///
    /// The [`convert`] method should not return numbers out of this bounds, which may cause
    /// unexpected errors. The default implementation returns `(0, None)` matches on any
    /// converters.
    ///
    /// [`None`]: core::option::Option::None
    /// [`convert`]: Self::convert
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Chaining two converters.
    ///
    /// If the first converter ended, the converter will provides outputs from the second
    /// converter.
    ///
    /// # Examples
    /// ```
    /// use conversion::converter::encoding::utf16::UTF16LEDecoder;
    /// use conversion::converter::ExactConverter;
    /// use conversion::iter::ConvertedIterator;
    /// use conversion::Converter;
    ///
    /// let iter = b"\xFF\xFE\x3D\xD8\xA3\xDC".into_iter().cloned();
    /// // UTF-16 with BOM
    /// let conv = ExactConverter::new([0xFF, 0xFE]).chain(UTF16LEDecoder::new());
    /// let decoded = ConvertedIterator::new(iter, conv);
    ///
    /// assert_eq!(Ok(String::from("💣")), decoded.collect());
    /// ```
    #[inline]
    fn chain<C>(self, other: C) -> ChainedConverter<Self, C>
    where
        C: Converter<Item = Self::Item, Output = Self::Output>,
        Self: Sized,
    {
        ChainedConverter::new(self, other)
    }
}
