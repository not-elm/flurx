use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::{AlreadyDroppedScheduler, FutureResult};
use crate::selector::Selector;
use crate::task::StateRef;

pub(crate) struct TaskFuture<'a, State, Selector, const SAFETY: bool> {
    pub(crate) selector: Selector,
    pub(crate) state: StateRef<'a, State>,
}


impl<'a, State, Sel> Future for TaskFuture<'a, State, Sel, true>
    where
        Sel: Selector<State>,
        State: Clone + 'a
{
    type Output = FutureResult<Sel::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.try_poll(cx) {
            Err(e) => Poll::Ready(Err(e)),
            Ok(Some(output)) => Poll::Ready(Ok(output)),
            Ok(None) => Poll::Pending
        }
    }
}

impl<'a, State, Sel> Future for TaskFuture<'a, State, Sel, false>
    where
        Sel: Selector<State>,
        State: Clone + 'a
{
    type Output = Sel::Output;

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

impl<'a, State, Sel, const SAFETY: bool> TaskFuture<'a, State, Sel, SAFETY>
    where
        Sel: Selector<State>,
        State: Clone + 'a
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

impl<'a, State, Sel> TaskFuture<'a, State, Sel, false>
    where
        Sel: Selector<State>
{
    #[allow(unused)]
    fn new_non_safety(state: StateRef<'a, State>, selector: Sel) -> TaskFuture<'a, State, Sel, false> {
        Self {
            selector,
            state,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::future::{block_on, poll_once};

    use crate::selector::wait;
    use crate::task::future::TaskFuture;

    #[test]
    fn count_up() {
        let mut state = UnsafeCell::new(None);
        let mut future = TaskFuture::new_non_safety(unsafe {
            &*state.get()
        }, wait::while_(|state| state == 1));

        *state.get_mut() = Some(0);
        assert!(block_on(poll_once(&mut future)).is_none());
        *state.get_mut() = Some(1);
        assert!(block_on(poll_once(&mut future)).is_some());
    }
}

