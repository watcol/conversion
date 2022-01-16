//! Infallible converters.

use core::convert::Infallible;

use crate::Converter;

/// A trait for infallible converters.
pub trait InfallibleConverter: Converter + sealed_converter::Sealed {
    /// Converting without any errors.
    fn convert_ok<E>(&mut self, item: Self::Item, buf: &mut E) -> usize
    where
        E: Extend<Self::Output>;

    /// Finalizing without any errors.
    fn finalize_ok(&mut self);
}

impl<C: Converter> InfallibleConverter for C
where
    C::Error: InfallibleError,
{
    #[inline]
    fn convert_ok<E>(&mut self, item: Self::Item, buf: &mut E) -> usize
    where
        E: Extend<Self::Output>,
    {
        self.convert(item, buf).unwrap_infallible()
    }

    #[inline]
    fn finalize_ok(&mut self) {
        self.finalize().unwrap_infallible()
    }
}

/// A marker trait for error types that will never constructed.
///
/// Implementing this on types wrap [`Infallible`] and never constructed marks the type as an
/// infallible error, and implement [`InfallibleConverter`] on converters which has the error
/// type.
///
/// [`Infallible`]: core::convert::Infallible
/// [`InfallibleConverter`]: self::InfallibleConverter
pub trait InfallibleError {}

impl InfallibleError for Infallible {}

/// A trait for [`Result`] types with errors implementing [`InfallibleError`].
///
/// [`Result`]: core::result::Result
/// [`InfallibleError`]: self::InfallibleError
pub trait InfallibleResult: sealed_result::Sealed {
    /// `T` of `Result<T, E>`.
    type T;

    /// Unwrapping the result type.
    ///
    /// If self has [`Err`], the behavior is undefined.
    ///
    /// [`Err`]: core::result::Result::Err
    fn unwrap_infallible(self) -> Self::T;
}

impl<T, E: InfallibleError> InfallibleResult for Result<T, E> {
    type T = T;

    fn unwrap_infallible(self) -> Self::T {
        unsafe {
            self.unwrap_unchecked()
        }
    }
}

mod sealed_converter {
    pub trait Sealed {}

    impl<C: crate::Converter> Sealed for C where C::Error: super::InfallibleError {}
}

mod sealed_result {
    pub trait Sealed {}

    impl<T, E: super::InfallibleError> Sealed for Result<T, E> {}
}
