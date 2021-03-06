//! Iterator support.
mod tryiter;
pub use tryiter::ConvertedTryIterator;

use crate::Converter;
use alloc::collections::VecDeque;

/// A wrapper for [`Iterator`], converts its item using [`Converter`].
///
/// # Example
/// ```
/// use conversion::iter::ConvertedIterator;
/// use conversion::converter::IterConverter;
///
/// let mut iter = ConvertedIterator::new("straße".chars(), IterConverter::new(char::to_uppercase));
///
/// assert_eq!(Some(Ok('S')), iter.next());
/// assert_eq!(Some(Ok('T')), iter.next());
/// assert_eq!(Some(Ok('R')), iter.next());
/// assert_eq!(Some(Ok('A')), iter.next());
/// assert_eq!(Some(Ok('S')), iter.next());
/// assert_eq!(Some(Ok('S')), iter.next());
/// assert_eq!(Some(Ok('E')), iter.next());
/// assert_eq!(None, iter.next());
/// ```
///
/// [`Iterator`]: core::iter::Iterator
/// [`Converter`]: crate::Converter
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl<I, C> Iterator for ConvertedIterator<I, C, C::Output>
where
    I: Iterator,
    C: Converter<Item = I::Item>,
{
    type Item = Result<C::Output, C::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            self.buffer.pop_front().map(Ok)
        } else {
            loop {
                match self.iter.next() {
                    Some(item) => match self.converter.convert(item, &mut self.buffer) {
                        Ok(0) if self.converter.is_ended() => match self.converter.finalize() {
                            Ok(()) => break None,
                            Err(e) => break Some(Err(e)),
                        },
                        Ok(0) => continue,
                        Ok(_) => break self.buffer.pop_front().map(Ok),
                        Err(e) => break Some(Err(e)),
                    },
                    None => match self.converter.finalize() {
                        Ok(()) => break None,
                        Err(e) => break Some(Err(e)),
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
