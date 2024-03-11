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
        if self.reactors.is_empty() {
            return Poll::Ready(());
        }

        while let Some(mut reactor) = self.reactors.pop() {
            if reactor.as_mut().poll(cx).is_pending() {
                self.polled.push(reactor);
            }
        }
        if self.polled.is_empty() {
            Poll::Ready(())
        } else {
            while let Some(f) = self.polled.pop() {
                self.reactors.push(f);
            }
            Poll::Pending
        }
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
        scheduler.schedule(|tc| async move {
            tc.task(once::run(|_| {
                tx.set(tx.get() + 1);
            })).await;
        });
        scheduler.schedule(|tc| async move {
            tc.task(once::run(|_| {
                tx2.set(tx2.get() + 1);
            })).await;
        });
        scheduler.run(0).await;
        assert_eq!(rx.get(), 2);
    }
}