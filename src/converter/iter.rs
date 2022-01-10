use core::{convert::Infallible, marker::PhantomData};

use crate::Converter;

/// Converting values with a function returns an iterator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IterConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F, I> From<F> for IterConverter<F, I> {
    #[inline]
    fn from(f: F) -> Self {
        Self {
            f,
            _phantomi: PhantomData,
        }
    }
}

impl<F, I> IterConverter<F, I> {
    /// Creating a new instance.
    #[inline]
    pub fn new(f: F) -> Self {
        Self::from(f)
    }
}

impl<F, I, Iter> Converter for IterConverter<F, I>
where
    Iter: Iterator,
    F: FnMut(I) -> Iter,
{
    type Item = I;
    type Output = Iter::Item;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend((self.f)(item));
        Ok(1)
    }
}

/// Converting values with a function returns a result of an iterator.
pub struct TryIterConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F, I> From<F> for TryIterConverter<F, I> {
    #[inline]
    fn from(f: F) -> Self {
        Self {
            f,
            _phantomi: PhantomData,
        }
    }
}

impl<F, I> TryIterConverter<F, I> {
    /// Creating a new instance.
    #[inline]
    pub fn new(f: F) -> Self {
        Self::from(f)
    }
}

impl<F, I, Iter, E> Converter for TryIterConverter<F, I>
where
    Iter: Iterator,
    F: FnMut(I) -> Result<Iter, E>,
{
    type Item = I;
    type Output = Iter::Item;
    type Error = E;

    fn convert<Ext>(&mut self, item: Self::Item, buf: &mut Ext) -> Result<usize, Self::Error>
    where
        Ext: Extend<Self::Output>,
    {
        buf.extend((self.f)(item)?);
        Ok(1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
