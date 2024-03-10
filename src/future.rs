use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::selector::StateSelector;
use crate::task::StateRef;

pub struct StateFuture<'a, State, Selector>
{
    selector: Selector,
    state: StateRef<'a, State>,
}


impl<'a, State, Selector> StateFuture<'a, State, Selector>
    where
        Selector: StateSelector<State>
{
    pub fn new(state: StateRef<'a, State>, selector: Selector) -> StateFuture<State, Selector> {
        Self {
            selector,
            state,
        }
    }
}

impl<'a, State, Selector> Future for StateFuture<'a, State, Selector>
    where
        Selector: StateSelector<State>
{
    type Output = Selector::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(output) = self.selector.clone().select(self.state) {
            Poll::Ready(output)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::future::{block_on, poll_once};

    use crate::future::StateFuture;

    #[test]
    fn count_up() {
        let mut state = UnsafeCell::new(0);
        let mut future = StateFuture::new(unsafe {
            &*state.get()
        }, |state: &i32| {
            if *state == 1 {
                Some(())
            } else {
                None
            }
        });
        assert!(block_on(poll_once(&mut future)).is_none());
        *state.get_mut() = 1;
        assert!(block_on(poll_once(&mut future)).is_some());
    }
}

