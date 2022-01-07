//! Asynchronous stream support.

mod trystream;
pub use trystream::ConvertedTryStream;

use crate::Converter;
use alloc::collections::VecDeque;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::{ready, Stream};
use pin_project_lite::pin_project;

pin_project! {
    /// A wrapper of [`Stream`], converts its items using [`TryConverter`].
    ///
    /// [`Stream`]: futures_core::stream::Stream
    /// [`TryConverter`]: crate::TryConverter
    #[derive(Debug)]
    pub struct ConvertedStream<S, C, O> {
        buffer: VecDeque<O>,
        #[pin]
        stream: S,
        converter: C,
    }
}

impl<S, C> ConvertedStream<S, C, C::Output>
where
    S: Stream,
    C: Converter<Item = S::Item>,
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

impl<S, C> Stream for ConvertedStream<S, C, C::Output>
where
    S: Stream,
    C: Converter<Item = S::Item>,
{
    type Item = Result<C::Output, C::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if !this.buffer.is_empty() {
            Poll::Ready(this.buffer.pop_front().map(Ok))
        } else {
            match ready!(this.stream.poll_next(cx)) {
                Some(item) => match this.converter.convert(item, this.buffer) {
                    Ok(0) => Poll::Pending,
                    Ok(_) => Poll::Ready(this.buffer.pop_front().map(Ok)),
                    Err(e) => Poll::Ready(Some(Err(e))),
                },
                None => Poll::Ready(None),
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