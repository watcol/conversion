use core::convert::Infallible;
use core::fmt;
use core::marker::PhantomData;

use crate::Converter;

/// Converting values with a function returns a type implements [`IntoIterator`].
///
/// # Examples
/// ```
/// use conversion::converter::IterConverter;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "straße".chars();
///
/// let uppered = ConvertedIterator::new(iter, IterConverter::new(char::to_uppercase));
///
/// assert_eq!(Ok(String::from("STRASSE")), uppered.collect());
/// ```
///
/// [`IntoIterator`]: core::iter::IntoIterator
pub struct IterConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F: Clone, I> Clone for IterConverter<F, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantomi: PhantomData,
        }
    }
}

impl<F: Copy, I> Copy for IterConverter<F, I> {}

impl<F: fmt::Debug, I> fmt::Debug for IterConverter<F, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterConverter").field(&self.f).finish()
    }
}

impl<F: PartialEq, I> PartialEq for IterConverter<F, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl<F: Eq, I> Eq for IterConverter<F, I> {}

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

impl<F, I, B> Converter for IterConverter<F, I>
where
    B: IntoIterator,
    F: FnMut(I) -> B,
{
    type Item = I;
    type Output = B::Item;
    type Error = Infallible;

    fn convert<E>(&mut self, item: Self::Item, buf: &mut E) -> Result<usize, Self::Error>
    where
        E: Extend<Self::Output>,
    {
        buf.extend((self.f)(item));
        Ok(1)
    }
}

/// Converting values with a function returns a result of a type implements [`IntoIterator`].
///
/// # Examples
/// ```
/// use conversion::converter::TryIterConverter;
/// use conversion::iter::ConvertedIterator;
///
/// let iter = "straße".chars();
///
/// let mut uppered = ConvertedIterator::new(iter, TryIterConverter::new(|c: char| if c.is_ascii() {
///     Ok(c.to_uppercase())
/// } else {
///     Err("This character is not ASCII.")
/// }));
///
/// assert_eq!(Some(Ok('S')), uppered.next());
/// assert_eq!(Some(Ok('T')), uppered.next());
/// assert_eq!(Some(Ok('R')), uppered.next());
/// assert_eq!(Some(Ok('A')), uppered.next());
/// assert_eq!(Some(Err("This character is not ASCII.")), uppered.next());
/// assert_eq!(Some(Ok('E')), uppered.next());
/// assert_eq!(None, uppered.next());
/// ```
///
/// [`IntoIterator`]: core::iter::IntoIterator
pub struct TryIterConverter<F, I> {
    f: F,
    _phantomi: PhantomData<I>,
}

impl<F: Clone, I> Clone for TryIterConverter<F, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantomi: PhantomData,
        }
    }
}

impl<F: Copy, I> Copy for TryIterConverter<F, I> {}

impl<F: fmt::Debug, I> fmt::Debug for TryIterConverter<F, I> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TryIterConverter").field(&self.f).finish()
    }
}

impl<F: PartialEq, I> PartialEq for TryIterConverter<F, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl<F: Eq, I> Eq for TryIterConverter<F, I> {}

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

impl<F, I, B, E> Converter for TryIterConverter<F, I>
where
    B: IntoIterator,
    F: FnMut(I) -> Result<B, E>,
{
    type Item = I;
    type Output = B::Item;
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
