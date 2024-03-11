use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::scheduler::Reactor;

pub(crate) struct ReactorsFuture<'a, 'b> {
    pub reactors: &'b mut Vec<Reactor<'a>>,
    pub polled: Vec<Reactor<'a>>,
}


impl<'a, 'b> Future for ReactorsFuture<'a, 'b>

{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut reactor) = self.reactors.pop() {
            if reactor.as_mut().poll(cx).is_pending() {
                self.polled.push(reactor);
            }
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            while let Some(f) = self.polled.pop() {
                self.reactors.push(f);
            }
            Poll::Ready(())
        }
    }
}