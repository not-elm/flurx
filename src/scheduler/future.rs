use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::scheduler::PinFuture;

pub(crate) struct SchedulerFuture<'a, 'b> {
    pub futures: &'b mut Vec<PinFuture<'a>>,
    pub polled: Vec<PinFuture<'a>>,
}


impl<'a, 'b> Future for SchedulerFuture<'a, 'b>

{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut future) = self.futures.pop() {
            if future.as_mut().poll(cx).is_pending() {
                self.polled.push(future);
            }
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            while let Some(f) = self.polled.pop() {
                self.futures.push(f);
            }
            Poll::Ready(())
        }
    }
}