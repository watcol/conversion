//! Infallible converters.

use core::convert::Infallible;

use crate::Converter;

/// A trait for infallible converters.
pub trait InfallibleConverter: Converter + sealed::Sealed {
    /// Converting without any errors.
    fn convert_ok<E>(&mut self, item: Self::Item, buf: &mut E) -> usize
    where
        E: Extend<Self::Output>,
    {
        self.convert(item, buf).unwrap_or(0)
    }
}

impl<C: Converter> InfallibleConverter for C where C::Error: InfallibleError {}

/// A marker trait for error types that will never constructed.
///
/// Implementing this on types wrap [`Infallible`] and never constructed marks the type as an
/// infallible error, and implement [`InfallibleConverter`] on converters which has the error
/// type.
pub trait InfallibleError {}

impl InfallibleError for Infallible {}

mod sealed {
    pub trait Sealed {}

    impl<C: crate::Converter> Sealed for C where C::Error: super::InfallibleError {}
}
