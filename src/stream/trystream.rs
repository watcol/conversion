use crate::{error::CombinedError, Converter};
use alloc::collections::VecDeque;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::{ready, Stream, TryStream};
use pin_project_lite::pin_project;

pin_project! {
    /// A wrapper of [`TryStream`], converts its items using [`Converter`].
    ///
    /// [`TryStream`]: futures_core::stream::TryStream
    /// [`Converter`]: crate::Converter
    #[derive(Debug)]
    pub struct ConvertedTryStream<S, C, O> {
        buffer: VecDeque<O>,
        #[pin]
        stream: S,
        converter: C,
    }
}

impl<S, C, O> ConvertedTryStream<S, C, O> {
    /// Creating a new instance.
    #[inline]
    pub fn new(stream: S, converter: C) -> Self {
        Self {
            buffer: VecDeque::new(),
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
                    Ok(0) => Poll::Pending,
                    Ok(_) => Poll::Ready(this.buffer.pop_front().map(Ok)),
                    Err(e) => Poll::Ready(Some(Err(CombinedError::Conversion(e)))),
                },
                Some(Err(e)) => Poll::Ready(Some(Err(CombinedError::Stream(e)))),
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
