use crate::{error::CombinedError, Converter};
use alloc::collections::VecDeque;

/// A wrapper for [`Iterator`] whose item is [`Result`], converts its item using
/// [`Converter`].
///
/// # Example
/// ```
/// use conversion::iter::ConvertedTryIterator;
/// use conversion::converter::TryMapConverter;
/// use conversion::error::CombinedError;
///
/// let iter = ["3", "0", "bad", "7"].into_iter().map(|s| s.parse::<i32>());
/// let divide_42 = TryMapConverter::new(|i| (42i32).checked_div(i).ok_or("division by zero"));
/// let mut converted = ConvertedTryIterator::new(iter, divide_42);
///
/// assert_eq!(Some(Ok(14)), converted.next());
/// assert_eq!(Some(Err(CombinedError::Conversion("division by zero"))), converted.next());
/// assert!(matches!(converted.next(), Some(Err(CombinedError::Stream(_)))));
/// assert_eq!(Some(Ok(6)), converted.next());
/// assert_eq!(None, converted.next());
/// ```
///
/// [`Iterator`]: core::iter::Iterator
/// [`Converter`]: crate::Converter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertedTryIterator<I, C, O> {
    buffer: VecDeque<O>,
    iter: I,
    converter: C,
}

impl<I, C, T, E> ConvertedTryIterator<I, C, C::Output>
where
    I: Iterator<Item = Result<T, E>>,
    C: Converter<Item = T>,
{
    /// Creating a new instance.
    #[inline]
    pub fn new<B>(iter: B, converter: C) -> Self
    where
        B: IntoIterator<IntoIter = I>,
    {
        let (min, max) = converter.size_hint();
        Self {
            buffer: VecDeque::with_capacity(max.unwrap_or(min)),
            iter: iter.into_iter(),
            converter,
        }
    }
}

impl<I, C, T, E> Iterator for ConvertedTryIterator<I, C, C::Output>
where
    I: Iterator<Item = Result<T, E>>,
    C: Converter<Item = T>,
{
    type Item = Result<C::Output, CombinedError<E, C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            self.buffer.pop_front().map(Ok)
        } else {
            loop {
                match self.iter.next() {
                    Some(Ok(item)) => match self.converter.convert(item, &mut self.buffer) {
                        Ok(0) if self.converter.is_ended() => match self.converter.finalize() {
                            Ok(()) => break None,
                            Err(e) => break Some(Err(CombinedError::Conversion(e))),
                        },
                        Ok(0) => continue,
                        Ok(_) => break self.buffer.pop_front().map(Ok),
                        Err(e) => break Some(Err(CombinedError::Conversion(e))),
                    },
                    Some(Err(e)) => break Some(Err(CombinedError::Stream(e))),
                    None => match self.converter.finalize() {
                        Ok(()) => break None,
                        Err(e) => break Some(Err(CombinedError::Conversion(e))),
                    },
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (iter_min, iter_max) = self.iter.size_hint();
        let (converter_min, converter_max) = self.converter.size_hint();
        (
            iter_min * converter_min,
            iter_max.zip(converter_max).map(|(x, y)| x * y),
        )
    }
}
