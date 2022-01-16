use core::convert::Infallible;
use core::fmt;
use core::marker::PhantomData;

use crate::Converter;

/// Converting values with a function.
///
/// [`TryInto`]: core::convert::TryInto
pub struct MapConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F: Clone, I> Clone for MapConverter<F, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantomi: PhantomData,
        }
    }
}

impl<F: Copy, I> Copy for MapConverter<F, I> {}

impl<F: fmt::Debug, I> fmt::Debug for MapConverter<F, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MapConverter").field(&self.f).finish()
    }
}

impl<F: PartialEq, I> PartialEq for MapConverter<F, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl<F: Eq, I> Eq for MapConverter<F, I> {}

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
        buf.extend([(self.f)(item)]);
        Ok(1)
    }

    #[inline]
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

impl<F: Clone, I> Clone for TryMapConverter<F, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantomi: PhantomData,
        }
    }
}

impl<F: Copy, I> Copy for TryMapConverter<F, I> {}

impl<F: fmt::Debug, I> fmt::Debug for TryMapConverter<F, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TryMapConverter").field(&self.f).finish()
    }
}

impl<F: PartialEq, I> PartialEq for TryMapConverter<F, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl<F: Eq, I> Eq for TryMapConverter<F, I> {}

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
        buf.extend([(self.f)(item)?]);
        Ok(1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(1))
    }
}
