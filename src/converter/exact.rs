use core::fmt;
use core::marker::PhantomData;

use crate::Converter;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct UnmatchedError;

impl fmt::Display for UnmatchedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An item unmatched.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnmatchedError {}

/// Consuming exact items that `I` provides, without emitting outputs.
///
/// # Examples
/// ```
/// use conversion::converter::ExactConverter;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "abc".chars();
/// let iter2 = "bcd".chars();
/// let conv = ExactConverter::<_, ()>::new(['a', 'b', 'c']);
/// let converted = ConvertedIterator::new(iter, conv.clone());
/// let converted2 = ConvertedIterator::new(iter2, conv);
///
/// assert_eq!(Ok(vec![]), converted.collect());
/// assert!(converted2.collect::<Result<Vec<_>, _>>().is_err());
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ExactConverter<I, O> {
    iter: I,
    is_ended: bool,
    _phantom: PhantomData<O>,
}

impl<T: IntoIterator, O> From<T> for ExactConverter<T::IntoIter, O> {
    #[inline]
    fn from(iter: T) -> Self {
        Self {
            iter: iter.into_iter(),
            is_ended: false,
            _phantom: PhantomData,
        }
    }
}

impl<I, O> ExactConverter<I, O> {
    /// Creating a new instance.
    #[inline]
    pub fn new<T: IntoIterator<IntoIter = I>>(iter: T) -> Self {
        Self::from(iter)
    }
}

impl<I: Iterator, O> Converter for ExactConverter<I, O>
where
    I::Item: PartialEq,
{
    type Item = I::Item;
    type Output = O;
    type Error = UnmatchedError;

    fn convert<E>(&mut self, item: Self::Item, _buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        if self.is_ended {
            return Ok(0);
        }

        match self.iter.next() {
            Some(i) if i == item => Ok(0),
            Some(_) => Err(UnmatchedError),
            None => {
                self.is_ended = true;
                Ok(0)
            }
        }
    }

    #[inline]
    fn is_ended(&self) -> bool {
        self.is_ended
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
