use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::scheduler::Reactor;

pub(crate) struct ReactorsFuture<'state, 'future> {
    pub reactors: &'future mut Vec<Reactor<'state>>,
    pub polled: Vec<Reactor<'state>>,
}


impl<'state, 'future> Future for ReactorsFuture<'state, 'future>

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
            Poll::Pending
        }
    }
}


impl<'state, 'future> Drop for ReactorsFuture<'state, 'future> {
    fn drop(&mut self) {
        while let Some(f) = self.polled.pop() {
            self.reactors.push(f);
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