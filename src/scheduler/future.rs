use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::scheduler::Reactor;

pub(crate) struct ReactorsFuture<'state, 'future> {
    pub reactor: &'future mut Option<Reactor<'state>>,
    pub tmp: &'future mut Option<Reactor<'state>>,
}


impl<'state, 'future> Future for ReactorsFuture<'state, 'future>

{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut reactor) = self.reactor.take() {
            if reactor.as_mut().poll(cx).is_pending() {
                self.tmp.replace(reactor);
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        } else {
            Poll::Ready(())
        }
    }
}

impl<'state, 'future> Drop for ReactorsFuture<'state, 'future> {
    #[inline(always)]
    fn drop(&mut self) {
        std::mem::swap(self.reactor, self.tmp);
    }
}


