//! An abstraction crate to convert iterators on the fly.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Types to convert N items into M outputs.
pub trait Converter {
    /// The type of input items.
    type Item;

    /// The type of outputs.
    type Output;

    /// The type of errors while conversion.
    type Error;

    /// Consuming one item, stores outputs into `buf` implements [`Extend`], and returns
    /// the number of stored outputs or a conversion error. If there are no outputs yet,
    /// you should store the item and return `Ok(0)`.
    ///
    /// [`Extend`]: core::iter::Extend
    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>;
}
