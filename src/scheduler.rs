use std::borrow::Cow;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::task::Task;

type PinFuture<'a> = Pin<Box<dyn Future<Output=()> + 'a>>;


pub struct Scheduler<'a, State>{
    state: Cow<'a, Option<State>>,
    futures: Vec<PinFuture<'a>>,
    _maker: PhantomData<&'a State>,
}


impl<'a, State> Scheduler<'a, State>
{
    pub fn schedule<F>(&mut self, f: impl FnOnce(Task<State>) -> F)
        where F: Future<Output=()> + 'a,
    {
        let task = Task::<'a, State>::new(&self.state);
        let future = f(task);
        self.futures.push(Box::pin(future));
    }

    pub async fn run(&mut self, state: State) {
        self.state.replace(state);
        let len = self.futures.len();
        SchedulerHandle {
            futures: &mut self.futures,
            polled: Vec::with_capacity(len),
        }
            .await
    }
}


impl<'a , State> Default for Scheduler<'a,  State> {
    fn default() -> Self {
        Self {
            state: Cow::Owned(None),
            futures: Vec::new(),
            _maker: PhantomData,
        }
    }
}

struct SchedulerHandle<'a, 'b> {
    futures: &'b mut Vec<PinFuture<'a>>,
    polled: Vec<PinFuture<'a>>,
}


impl<'a, 'b> Future for SchedulerHandle<'a, 'b>

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


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::scheduler::Scheduler;

    #[tokio::test]
    async fn one_until() {
        let mut scheduler = Scheduler::<i32>::default();
        let result = Arc::new(Mutex::new(false));
        let result2 = result.clone();
        scheduler.schedule(|task| async move {
            task.until(|state: &i32| {
                println!("state = {state}");
                *state < 2
            }).await;
            *result2.lock().unwrap() = true;
        });

        scheduler.run(1).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(!*result.lock().unwrap());

       scheduler.run(2).await;
        assert!(*result.lock().unwrap());
    }
}


