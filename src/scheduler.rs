use std::future::Future;
use std::pin::Pin;

use crate::scheduler::future::SchedulerFuture;
use crate::scheduler::state_ptr::StatePtr;
use crate::task::TaskCreator;

mod future;
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
    pub const fn new() -> Scheduler<'a, 'b, State> {
        Self {
            state: StatePtr::uninit(),
            futures: Vec::new(),
        }
    }

    pub fn schedule<F>(&mut self, f: impl FnOnce(TaskCreator<'a, State>) -> F)
        where F: Future<Output=()> + 'b,
    {
        self.futures.push(Box::pin(f(TaskCreator {
            state: self.state.state_ref()
        })));
    }

    pub async fn run(&mut self, state: State) {
        self.state.set(state);

        let len = self.futures.len();
        SchedulerFuture {
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
    use crate::selector::wait;

    #[tokio::test]
    async fn one_until() {
        let mut scheduler = Scheduler::<i32>::default();
        let result = Arc::new(Mutex::new(false));
        let r = result.clone();

        scheduler.schedule(|task| async move {
            task.task(wait::until(|state: &i32| {
                *state < 2
            })).await;
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

        scheduler.schedule(|tc| async move {
            tc.task(wait::while_(|state: &i32| {
                *state == 2
            })).await;

            tc.task(wait::until(|state: &i32| {
                *state < 3
            })).await;

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


