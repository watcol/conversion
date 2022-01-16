use core::fmt;
use core::marker::PhantomData;

use crate::Converter;

/// Converting values with [`TryInto`] trait.
///
/// # Examples
/// ```
/// use conversion::converter::IntoConverter;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = [0x2764, 0x110000];
/// let mut decoded = ConvertedIterator::new(iter, IntoConverter::<u32, char>::new());
///
/// assert_eq!(Some(Ok('❤')), decoded.next());
/// assert!(matches!(decoded.next(), Some(Err(_))));
/// assert_eq!(None, decoded.next());
/// ```
///
/// [`TryInto`]: core::convert::TryInto
pub struct IntoConverter<I, O> {
    _phantomi: PhantomData<I>,
    _phantomo: PhantomData<O>,
}

impl<I, O> Clone for IntoConverter<I, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<I, O> Copy for IntoConverter<I, O> {}

impl<I, O> fmt::Debug for IntoConverter<I, O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoConverter").finish()
    }
}

impl<I, O> PartialEq for IntoConverter<I, O> {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<I, O> Eq for IntoConverter<I, O> {}

impl<I, O> Default for IntoConverter<I, O> {
    #[inline]
    fn default() -> Self {
        Self {
            _phantomi: PhantomData,
            _phantomo: PhantomData,
        }
    }
}

impl<I, O> IntoConverter<I, O> {
    /// Creating a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<I, O> Converter for IntoConverter<I, O>
where
    I: TryInto<O>,
{
    type Item = I;
    type Output = O;
    type Error = I::Error;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend([item.try_into()?]);
        Ok(1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
