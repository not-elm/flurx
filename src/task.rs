use crate::error::FutureResult;
use crate::selector::Selector;
use crate::task::future::TaskFuture;

pub mod future;

pub(in crate) type StateRef<'state, State> = &'state Option<State>;

pub struct ReactiveTask<'state, State> {
    pub(in crate) state: StateRef<'state, State>,
}

impl<'state, State> ReactiveTask<'state, State> {
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
    /// scheduler.schedule(|task|async move{
    ///     task.will(once::run(|state: usize|{
    ///         state
    ///     })).await;
    /// });
    /// ```
    #[inline]
    pub async fn will<Out, Sel>(&self, selector: Sel) -> Out
        where Sel: Selector<State, Output=Out>,
              State: Clone + 'state
    {
        TaskFuture::<State, Sel, false> {
            state: self.state,
            selector,
        }
            .await
    }
    
    /// This method will not be made public as the specifications have not yet been finalized.
    #[allow(unused)]
    pub(in crate) async fn try_will<Out, Sel>(&self, selector: Sel) -> FutureResult<Out>
        where Sel: Selector<State, Output=Out>,
              State: Clone + 'state
    {
        TaskFuture::<State, Sel, true> {
            state: self.state,
            selector,
        }
            .await
    }
}

#[allow(clippy::missing_trait_methods)]
impl<'state, State> Clone for ReactiveTask<'state, State> {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl<'state, State> Copy for ReactiveTask<'state, State> {}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::pin;

    use crate::task::ReactiveTask;
    use crate::tests::poll_once_block;

    #[test]
    fn once_wait() {
        let mut state = UnsafeCell::new(None);
        let task = ReactiveTask {
            state: unsafe { &*state.get() }
        };
        let f = task.will(|state: i32| {
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