//! Error types for this crate.

use crate::infallible::InfallibleError;
use core::fmt;

/// An error type for [`ConvertedTryIterator`] and [`ConvertedTryStream`].
///
/// [`ConvertedTryIterator`]: crate::iterator::ConvertedTryIterator
/// [`ConvertedTryStream`]: crate::stream::ConvertedTryStream
#[derive(Debug)]
pub enum CombinedError<S, C> {
    Stream(S),
    Conversion(C),
}

impl<S: fmt::Display, C: fmt::Display> fmt::Display for CombinedError<S, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CombinedError::Stream(e) => write!(f, "{}", e),
            CombinedError::Conversion(e) => write!(f, "{}", e),
        }
    }
}

impl<S: InfallibleError, C: InfallibleError> InfallibleError for CombinedError<S, C> {}

#[cfg(feature = "std")]
impl<S, C> std::error::Error for CombinedError<S, C>
where
    S: std::error::Error + 'static,
    C: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CombinedError::Stream(e) => Some(e),
            CombinedError::Conversion(e) => Some(e),
        }
    }
}
