use crate::error::CombinedError;
use crate::Converter;
use alloc::collections::VecDeque;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::{ready, Stream, TryStream};
use pin_project_lite::pin_project;

pin_project! {
    /// A wrapper of [`TryStream`], converts its items using [`Converter`].
    ///
    /// # Example
    /// ```
    /// use conversion::stream::ConvertedTryStream;
    /// use conversion::converter::TryMapConverter;
    /// use conversion::error::CombinedError;
    /// use futures::stream::{self, StreamExt};
    ///
    /// # futures::executor::block_on(async {
    /// let stream = stream::iter(["3", "0", "bad", "7"].into_iter().map(|s| s.parse::<i32>()));
    /// let divide_42 = TryMapConverter::new(|i| (42i32).checked_div(i).ok_or("division by zero"));
    /// let mut converted = ConvertedTryStream::new(stream, divide_42);
    ///
    /// assert_eq!(Some(Ok(14)), converted.next().await);
    /// assert_eq!(Some(Err(CombinedError::Conversion("division by zero"))), converted.next().await);
    /// assert!(matches!(converted.next().await, Some(Err(CombinedError::Stream(_)))));
    /// assert_eq!(Some(Ok(6)), converted.next().await);
    /// assert_eq!(None, converted.next().await);
    ///
    /// # });
    /// ```
    ///
    /// [`TryStream`]: futures_core::stream::TryStream
    /// [`Converter`]: crate::Converter
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ConvertedTryStream<S, C, O> {
        buffer: VecDeque<O>,
        #[pin]
        stream: S,
        converter: C,
    }
}

impl<S, C> ConvertedTryStream<S, C, C::Output>
where
    S: TryStream,
    C: Converter<Item = S::Ok>,
{
    /// Creating a new instance.
    #[inline]
    pub fn new(stream: S, converter: C) -> Self {
        let (min, max) = converter.size_hint();
        Self {
            buffer: VecDeque::with_capacity(max.unwrap_or(min)),
            stream,
            converter,
        }
    }
}

impl<S, C> Stream for ConvertedTryStream<S, C, C::Output>
where
    S: TryStream,
    C: Converter<Item = S::Ok>,
{
    type Item = Result<C::Output, CombinedError<S::Error, C::Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if !this.buffer.is_empty() {
            Poll::Ready(this.buffer.pop_front().map(Ok))
        } else {
            match ready!(this.stream.try_poll_next(cx)) {
                Some(Ok(item)) => match this.converter.convert(item, this.buffer) {
                    Ok(0) if this.converter.is_ended() => match this.converter.finalize() {
                        Ok(()) => Poll::Ready(None),
                        Err(e) => Poll::Ready(Some(Err(CombinedError::Conversion(e)))),
                    },
                    Ok(0) => Poll::Pending,
                    Ok(_) => Poll::Ready(this.buffer.pop_front().map(Ok)),
                    Err(e) => Poll::Ready(Some(Err(CombinedError::Conversion(e)))),
                },
                Some(Err(e)) => Poll::Ready(Some(Err(CombinedError::Stream(e)))),
                None => match this.converter.finalize() {
                    Ok(()) => Poll::Ready(None),
                    Err(e) => Poll::Ready(Some(Err(CombinedError::Conversion(e)))),
                },
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (stream_min, stream_max) = self.stream.size_hint();
        let (converter_min, converter_max) = self.converter.size_hint();
        (
            stream_min * converter_min,
            stream_max.zip(converter_max).map(|(x, y)| x * y),
        )
    }
}
