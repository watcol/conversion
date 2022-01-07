//! Iterator support.
mod tryiterator;
pub use tryiterator::ConvertedTryIterator;

use crate::Converter;
use alloc::collections::VecDeque;

/// A wrapper for [`Iterator`], converts its item using [`Converter`].
///
/// [`Iterator`]: core::iter::Iterator
/// [`Converter`]: crate::Converter
pub struct ConvertedIterator<I, C, O> {
    buffer: VecDeque<O>,
    iter: I,
    converter: C,
}

impl<I, C> ConvertedIterator<I, C, C::Output>
where
    I: Iterator,
    C: Converter<Item = I::Item>,
{
    /// Creating a new instance.
    pub fn new(iter: I, converter: C) -> Self {
        let (min, max) = converter.size_hint();
        Self {
            buffer: VecDeque::with_capacity(max.unwrap_or(min)),
            iter,
            converter,
        }
    }
}

impl<I, C> Iterator for ConvertedIterator<I, C, C::Output>
where
    I: Iterator,
    C: Converter<Item = I::Item>,
{
    type Item = Result<C::Output, C::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            self.buffer.pop_front().map(Ok)
        } else {
            loop {
                match self.iter.next() {
                    Some(item) => match self.converter.convert(item, &mut self.buffer) {
                        Ok(0) => continue,
                        Ok(_) => break self.buffer.pop_front().map(Ok),
                        Err(e) => break Some(Err(e)),
                    },
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
