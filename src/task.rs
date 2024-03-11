use crate::error::FutureResult;
use crate::selector::Selector;
use crate::task::future::TaskFuture;

pub mod future;

pub(crate) type StateRef<'a, State> = &'a Option<State>;

pub struct TaskCreator<'a, State> {
    pub(crate) state: StateRef<'a, State>,
}

impl<'a, State> TaskCreator<'a, State> {
    /// Create a new task.
    ///
    /// Several [`Selector`](crate::selector::Selector)s are provided by default, but you can also define your own.
    ///
    /// The default [`Selector`](crate::selector::Selector)s are as follows.
    ///
    /// * [`wait::until`](crate::prelude::wait::until)
    /// * [`wait::while`](crate::prelude::wait::while_)
    /// * [`once::run`](crate::prelude::once::run)
    /// * [`repeat::count`](crate::prelude::repeat::count)
    /// * [`repeat::forever`](crate::prelude::repeat::forever)
    /// * [`delay::time`](crate::prelude::delay::time)
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use flurx::prelude::once;
    /// use flurx::Scheduler;
    ///
    /// let mut scheduler = Scheduler::<usize>::new();
    /// scheduler.schedule(|tc|async move{
    ///     tc.task(once::run(|state: usize|{
    ///         state
    ///     })).await;
    /// });
    /// ```
    pub async fn task<Out, Sel>(&self, selector: Sel) -> Out
        where Sel: Selector<State, Output=Out>,
              State: Clone + 'a
    {
        TaskFuture::<State, Sel, false> {
            state: self.state,
            selector,
        }
            .await
    }
    
    /// This method will not be made public as the specifications have not yet been finalized.
    #[allow(unused)]
    pub(crate) async fn try_task<Out, Sel>(&self, selector: Sel) -> FutureResult<Out>
        where Sel: Selector<State, Output=Out>,
              State: Clone + 'a
    {
        TaskFuture::<State, Sel, true> {
            state: self.state,
            selector,
        }
            .await
    }
}

impl<'a, State> Clone for TaskCreator<'a, State> {
    fn clone(&self) -> Self { *self }
}

impl<'a, State> Copy for TaskCreator<'a, State> {}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::pin;

    use crate::task::TaskCreator;
    use crate::tests::poll_once_block;

    #[test]
    fn once_wait() {
        let mut state = UnsafeCell::new(None);
        let task = TaskCreator {
            state: unsafe { &*state.get() }
        };
        let f = task.task(|state: i32| {
            if state == 1 {
                Some(())
            } else {
                None
            }
        });
        pin!(f);
        *state.get_mut() = Some(0);
        assert!(poll_once_block(&mut f).is_none());
        *state.get_mut() = Some(1);
        assert!(poll_once_block(&mut f).is_some());
    }
}