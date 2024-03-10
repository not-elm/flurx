use std::future::Future;
use std::pin::Pin;

use crate::scheduler::handle::SchedulerHandle;
use crate::scheduler::state_ptr::StatePtr;
use crate::task::Task;

mod handle;
mod state_ptr;

type PinFuture<'a> = Pin<Box<dyn Future<Output=()> + 'a>>;


#[derive(Default)]
pub struct Scheduler<'a, 'b, State> {
    state: StatePtr<'a, State>,
    futures: Vec<PinFuture<'b>>,
}


impl<'a, 'b, State> Scheduler<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b
{
    pub fn schedule<F>(&mut self, f: impl FnOnce(Task<'a, State>) -> F)
        where F: Future<Output=()> + 'b,
    {
        let task = Task::<'a, State>::new(self.state.state_ref());
        let future = f(task);
        self.futures.push(Box::pin(future));
    }

    pub async fn run(&mut self, state: State) {
        self.state.set(state);

        let len = self.futures.len();
        SchedulerHandle {
            futures: &mut self.futures,
            polled: Vec::with_capacity(len),
        }
            .await
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
        let r = result.clone();

        scheduler.schedule(|task| async move {
            task.wait_until(|state: &i32| {
                *state < 2
            }).await;
            *r.lock().unwrap() = true;
        });

        scheduler.run(1).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(*result.lock().unwrap());
    }


    #[tokio::test]
    async fn one_while() {
        let mut scheduler = Scheduler::<i32>::default();
        let result = Arc::new(Mutex::new(false));
        let r = result.clone();

        scheduler.schedule(|task| async move {
            task.wait_while(|state: &i32| {
                *state == 2
            }).await;
            *r.lock().unwrap() = true;
        });

        scheduler.run(1).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(*result.lock().unwrap());
    }

    #[tokio::test]
    async fn while_then_until() {
        let mut scheduler = Scheduler::<i32>::default();
        let result = Arc::new(Mutex::new(false));
        let r = result.clone();

        scheduler.schedule(|task| async move {
            task.wait_while(|state: &i32| {
                *state == 2
            }).await;
            task.wait_until(|state: &i32| {
                *state < 3
            }).await;
            *r.lock().unwrap() = true;
        });

        scheduler.run(1).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(3).await;
        assert!(*result.lock().unwrap());
    }
}


