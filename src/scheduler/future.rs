use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::scheduler::Reactor;

pub(crate) struct ReactorsFuture<'state, 'future> {
    pub reactor: &'future mut Option<Reactor<'state>>
}


impl<'state, 'future> Future for ReactorsFuture<'state, 'future>

{
    type Output = ();

    #[inline(always)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // if let Some(mut reactor) = self.reactor.take() {
        //     if reactor.as_mut().poll(cx).is_pending() {
        //         self.as_mut().reactor.replace(reactor);
        //     }
        // }
        if self
            .reactor
            .as_mut()
            .map(|reactor|reactor.as_mut().poll(cx).is_ready())
            .unwrap_or(false){
            self.reactor.take();
        }
 
        Poll::Ready(())
    }
}


