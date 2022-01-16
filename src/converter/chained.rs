use core::fmt;

use crate::Converter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChainedError<E, F> {
    First(E),
    Second(F),
}

impl<E: fmt::Display, F: fmt::Display> fmt::Display for ChainedError<E, F> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First(e) => write!(f, "{}", e),
            Self::Second(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(feature = "std")]
impl<E, F> std::error::Error for ChainedError<E, F>
where
    E: std::error::Error + 'static,
    F: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::First(e) => Some(e),
            Self::Second(e) => Some(e),
        }
    }
}

/// A converter for [`chain`] method.
///
/// [`chain`]: crate::Converter::chain
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ChainedConverter<C, D> {
    first: C,
    second: D,
    first_ended: bool,
}

impl<C, D> ChainedConverter<C, D> {
    /// Creating a new instance.
    #[inline]
    pub fn new(first: C, second: D) -> Self {
        Self {
            first,
            second,
            first_ended: false,
        }
    }
}

impl<C, D> Converter for ChainedConverter<C, D>
where
    C: Converter,
    C::Item: Clone,
    D: Converter<Item = C::Item, Output = C::Output>,
{
    type Item = C::Item;
    type Output = C::Output;
    type Error = ChainedError<C::Error, D::Error>;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        if self.first_ended {
            return self.second.convert(item, buf).map_err(ChainedError::Second);
        }

        match self.first.convert(item.clone(), buf) {
            Ok(0) if self.first.is_ended() => {
                self.first_ended = true;
                self.first.finalize().map_err(ChainedError::First)?;
                self.second.convert(item, buf).map_err(ChainedError::Second)
            }
            other => other.map_err(ChainedError::First),
        }
    }

    #[inline]
    fn is_ended(&self) -> bool {
        self.second.is_ended()
    }

    #[inline]
    fn finalize(&mut self) -> Result<(), Self::Error> {
        self.second.finalize().map_err(ChainedError::Second)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
