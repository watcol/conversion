//! An abstraction crate to convert iterators on the fly.
//!
//! # Demo
//! ```ignore
//! use conversion::{
//!     converter::{
//!         encoding::utf8::{UTF8Decoder, UTF8Encoder},
//!         MapConverter,
//!     },
//!     iterator::{ConvertedIterator, ConvertedTryIterator},
//! };
//!
//! // An original byte string.
//! let iter = b"\xf0\x9d\x84\x9emusic".into_iter().cloned();
//!
//! // Decoding UTF-8 byte string.
//! let mut decoded = ConvertedIterator::new(iter, UTF8Decoder::new());
//! // Convert to uppercase. (use ConvertedTryIterator because `decoded` returns Result items.)
//! let mut uppered = ConvertedTryIterator::new(deocded, MapConverter::new(char::to_ascii_uppercase));
//!
//! // Clone it because it will be consumed.
//! let uppered2 = uppered.clone();
//!
//! assert_eq!(Some(Ok('𝄞')), uppered.next());
//! assert_eq!(Some(Ok('m')), uppered.next());
//! assert_eq!(Some(Ok('u')), uppered.next());
//! assert_eq!(Some(Ok('s')), uppered.next());
//! assert_eq!(Some(Ok('i')), uppered.next());
//! assert_eq!(Some(Ok('c')), uppered.next());
//! assert_eq!(None, uppered.next());
//!
//! // Re-encode the value.
//! let encoded: Result<Vec<u8>, _> = ConvertedTryIterator::new(uppered2, UTF8Encoder::new()).collect();
//!
//! assert_eq!(b"\xf0\x9d\x84\x9emusic".to_vec(), encoded.unwrap());
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod infallible;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod iterator;
#[cfg(feature = "async")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "async")))]
pub mod stream;

pub mod converter;

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
}
