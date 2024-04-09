use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::scheduler::Reactor;

pub(crate) struct ReactorsFuture<'state, 'future> {
    pub reactor: &'future mut Option<Reactor<'state>>,
    pub tmp: &'future mut Option<Reactor<'state>>
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


#[cfg(test)]
mod tests {
    use crate::prelude::once;
    use crate::Scheduler;
    use crate::tests::result_event;

    #[tokio::test]
    async fn run_all_reactors() {
        let mut scheduler = Scheduler::new();
        let (tx, rx) = result_event::<usize>();
        let tx2 = tx.clone();
        scheduler.schedule(|task| async move {
            task.will(once::run(|_| {
                tx.set(tx.get() + 1);
            })).await;
        });
        scheduler.schedule(|task| async move {
            task.will(once::run(|_| {
                tx2.set(tx2.get() + 1);
            })).await;
        });
        scheduler.run(0).await;
        assert_eq!(rx.get(), 2);
    }
}