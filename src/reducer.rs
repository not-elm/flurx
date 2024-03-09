use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::dispatch::Dispatch;
use crate::scheduler::Scheduler;
use crate::store::Store;

type PinFuture<'b> = Pin<Box<dyn Future<Output=()> + 'b>>;


pub struct Reducer<'a, State> {
    store: &'a mut Store<State>,
    futures: Vec<PinFuture<'a>>,
}


impl<'a, State> Reducer<'a, State>
    where State: Default + 'static
{
    pub fn new(store: &'a mut Store<State>) -> Reducer<'a, State> {
        Self {
            store,
            futures: Vec::new(),
        }
    }

    pub fn schedule<F>(&mut self, f: impl FnOnce(Scheduler<'a, State>) -> F)
        where F: Future<Output=()> + 'a,

    {
        let scheduler = Scheduler::new(unsafe {
            self.store.unsafe_ref()
        });
        let future = f(scheduler);
        self.futures.push(Box::pin(future));
    }


    pub async fn dispatch(&mut self, dispatch: impl Dispatch<State>) {
        let state = mem::take(self.store.ref_mut());
        *self.store.ref_mut() = dispatch.dispatch(state);
        let len = self.futures.len();

        DispatchHandle {
            futures: &mut self.futures,
            polled: Vec::with_capacity(len),
        }
            .await
    }
}


struct DispatchHandle<'a, 'b> {
    futures: &'b mut Vec<PinFuture<'a>>,
    polled: Vec<PinFuture<'a>>,
}


impl<'a, 'b> Future for DispatchHandle<'a, 'b> {
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
    use crate::reducer::Reducer;
    use crate::selector::Until;
    use crate::store::Store;

    #[tokio::test]
    async fn one_until() {
        let mut store = Store::<usize>::default();
        let mut reducer = Reducer::new(&mut store);

        reducer.schedule(|scheduler| async move {
            let t1 = scheduler.add(Until::new(|state: &usize| {
                *state < 2
            }));
        });

        reducer.dispatch(|state| {
            state + 1
        })
            .await;
    }
}


