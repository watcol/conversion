use crate::{error::CombinedError, Converter};
use alloc::collections::VecDeque;

/// A wrapper for [`Iterator`] whose item is [`Result`], converts its item using
/// [`Converter`].
///
/// [`Iterator`]: core::iter::Iterator
/// [`Converter`]: crate::Converter
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
    pub fn new(iter: I, converter: C) -> Self {
        let (min, max) = converter.size_hint();
        Self {
            buffer: VecDeque::with_capacity(max.unwrap_or(min)),
            iter,
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
        if self.buffer.is_empty() {
            self.buffer.pop_front().map(Ok)
        } else {
            loop {
                match self.iter.next() {
                    Some(Ok(item)) => match self.converter.convert(item, &mut self.buffer) {
                        Ok(0) => continue,
                        Ok(_) => break self.buffer.pop_front().map(Ok),
                        Err(e) => break Some(Err(CombinedError::Conversion(e))),
                    },
                    Some(Err(e)) => break Some(Err(CombinedError::Stream(e))),
                    None => break None,
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
