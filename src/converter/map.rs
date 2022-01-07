use core::{convert::Infallible, marker::PhantomData};

use crate::Converter;

/// Converting values with a function.
///
/// [`TryInto`]: core::convert::TryInto
pub struct MapConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F, I> From<F> for MapConverter<F, I> {
    #[inline]
    fn from(f: F) -> Self {
        Self {
            f,
            _phantomi: PhantomData,
        }
    }
}

impl<F, I> MapConverter<F, I> {
    /// Creating a new instance.
    #[inline]
    pub fn new(f: F) -> Self {
        Self::from(f)
    }
}

impl<F, I, O> Converter for MapConverter<F, I>
where
    F: FnMut(I) -> O,
{
    type Item = I;
    type Output = O;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend(Some((self.f)(item)));
        Ok(1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}

/// Converting values with a failable function.
///
/// [`TryInto`]: core::convert::TryInto
pub struct TryMapConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F, I> From<F> for TryMapConverter<F, I> {
    #[inline]
    fn from(f: F) -> Self {
        Self {
            f,
            _phantomi: PhantomData,
        }
    }
}

impl<F, I> TryMapConverter<F, I> {
    /// Creating a new instance.
    #[inline]
    pub fn new(f: F) -> Self {
        Self::from(f)
    }
}

impl<F, I, O, E> Converter for TryMapConverter<F, I>
where
    F: FnMut(I) -> Result<O, E>,
{
    type Item = I;
    type Output = O;
    type Error = E;

    fn convert<Ext>(&mut self, item: Self::Item, buf: &mut Ext) -> Result<usize, Self::Error>
    where
        Ext: Extend<Self::Output>,
    {
        buf.extend(Some((self.f)(item)?));
        Ok(1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
