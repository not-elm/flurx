use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::error::{AlreadyDroppedScheduler, FutureResult};
use crate::selector::Selector;
use crate::task::StateRef;

pub(in crate) struct TaskFuture<'state, State, Selector, const SAFETY: bool> {
    pub(in crate) selector: Selector,
    pub(in crate) state: StateRef<'state, State>,
}


impl<'state, State, Sel> Future for TaskFuture<'state, State, Sel, true>
    where
        Sel: Selector<State>,
        State: Clone + 'state
{
    type Output = FutureResult<Sel::Output>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.try_poll(cx) {
            Err(e) => Poll::Ready(Err(e)),
            Ok(Some(output)) => Poll::Ready(Ok(output)),
            Ok(None) => Poll::Pending
        }
    }
}

impl<'state, State, Sel> Future for TaskFuture<'state, State, Sel, false>
    where
        Sel: Selector<State>,
        State: Clone + 'state
{
    type Output = Sel::Output;

    #[allow(clippy::panic)]
    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.try_poll(cx) {
            Err(e) => {
                panic!("{e}");
            }
            Ok(Some(output)) => Poll::Ready(output),
            Ok(None) => Poll::Pending
        }
    }
}

impl<'state, State, Sel, const SAFETY: bool> TaskFuture<'state, State, Sel, SAFETY>
    where
        Sel: Selector<State>,
        State: Clone + 'state
{
    fn try_poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Result<Option<Sel::Output>, AlreadyDroppedScheduler> {
        let state = self.state.as_ref().ok_or(AlreadyDroppedScheduler)?.clone();
        if let Some(output) = self.selector.select(state) {
            Ok(Some(output))
        } else {
            cx.waker().wake_by_ref();
            Ok(None)
        }
    }
}

impl<'state, State, Sel> TaskFuture<'state, State, Sel, false>
    where
        Sel: Selector<State>
{
    #[allow(unused)]
    fn new_non_safety(state: StateRef<'state, State>, selector: Sel) -> TaskFuture<'state, State, Sel, false> {
        Self {
            selector,
            state,
        }
    }
}
