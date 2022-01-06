//! An abstraction crate to convert iterators on the fly.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod infallible;

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
    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>;

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
