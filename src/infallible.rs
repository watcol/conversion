//! Infallible converters.

use core::convert::Infallible;

use crate::Converter;

/// A trait for infallible converters.
pub trait InfallibleConverter: Converter + sealed_converter::Sealed {
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
///
/// [`Infallible`]: core::convert::Infallible
/// [`InfallibleConverter`]: self::InfallibleConverter
pub trait InfallibleError {}

impl InfallibleError for Infallible {}

/// A trait to unwrapping [`Result`] types with errors implementing [`InfallibleError`] safely.
///
/// [`Result`]: core::result::Result
/// [`InfallibleError`]: self::InfallibleError
pub trait UnwrappableResult: sealed_result::Sealed {
    /// `T` of `Result<T, E>`.
    type T;

    /// Unwrapping the result with no concern.
    ///
    /// If self has [`Err`], the behavior is undefined.
    ///
    /// [`Err`]: core::result::Result::Err
    fn unwrap_safe(self) -> Self::T;
}

impl<T, E: InfallibleError> UnwrappableResult for Result<T, E> {
    type T = T;

    fn unwrap_safe(self) -> Self::T {
        #[cfg(feature = "nightly")]
        unsafe {
            self.unwrap_unchecked()
        }
        #[cfg(not(feature = "nightly"))]
        {
            debug_assert!(self.is_ok());
            match self {
                Ok(t) => t,
                Err(_) => unsafe { core::hint::unreachable_unchecked() },
            }
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
