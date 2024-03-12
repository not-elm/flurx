use std::future::Future;
use std::pin::Pin;

use crate::scheduler::future::ReactorsFuture;
use crate::scheduler::state_ptr::StatePtr;
use crate::task::ReactiveTask;

mod future;
mod state_ptr;

pub(crate) type Reactor<'a> = Pin<Box<dyn Future<Output=()> + 'a>>;


#[derive(Default)]
pub struct Scheduler<'a, 'b, State> {
    state: StatePtr<'a, State>,
    reactors: Vec<Reactor<'b>>,
}


impl<'a, 'b, State> Scheduler<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b
{
    #[must_use]
    /// Creates the empty scheduler.
    pub const fn new() -> Scheduler<'a, 'b, State> {
        Self {
            state: StatePtr::uninit(),
            reactors: Vec::new(),
        }
    }

    pub fn pending_reactors_count(&self) -> usize {
        self.reactors.len()
    }

    pub fn not_exists_pending(&self) -> bool {
        self.pending_reactors_count() == 0
    }


    /// Schedule the new [`Reactor`].
    ///
    /// The reality [`Reactor`] is [`Future`], it is polled once every time [`Scheduler::run`] is called.
    ///
    /// ## Examples
    /// ```no_run
    /// use flurx::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main(){
    ///     let mut scheduler = Scheduler::<usize>::new();
    ///     scheduler.schedule(|task|async move{
    ///         // (1)
    ///         task.will(wait::until(|state: usize|{
    ///             state < 2
    ///         })).await;
    ///     });
    ///     // state is 0, (1) returns [`Future::Pending`].
    ///     scheduler.run(0).await;
    ///     // state is 1, (1) returns [`Future::Pending`].
    ///     scheduler.run(1).await;
    ///     // state is 2, (1) returns [`Future::Ready(2)`].
    ///     scheduler.run(2).await;
    /// }
    /// ```
    pub fn schedule<F>(&mut self, f: impl FnOnce(ReactiveTask<'a, State>) -> F)
        where F: Future<Output=()> + 'b,
    {
        self.reactors.push(Box::pin(f(ReactiveTask {
            state: self.state.state_ref()
        })));
    }

    /// Poll all registered features once each.
    pub async fn run(&mut self, state: State) {
        self.state.set(state);

        let len = self.reactors.len();
        ReactorsFuture {
            reactors: &mut self.reactors,
            polled: Vec::with_capacity(len),
        }
            .await;
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
            task.will(wait::until(|state| {
                state == 2
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

        scheduler.schedule(|task| async move {
            task.will(wait::until(|state| {
                3 <= state
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


