use core::marker::PhantomData;

use crate::Converter;

/// Converting values with [`TryInto`] trait.
///
/// [`TryInto`]: core::convert::TryInto
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntoConverter<I, O> {
    _phantomi: PhantomData<I>,
    _phantomo: PhantomData<O>,
}

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
