use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::selector::Selector;
use crate::task::StateRef;

pub struct StateFuture<'a, State, Selector> {
    pub(crate) selector: Selector,
    pub(crate) state: StateRef<'a, State>,
}


impl<'a, State, Sel> Future for StateFuture<'a, State, Sel>
    where
        Sel: Selector<State>
{
    type Output = Sel::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(output) = self.selector.select(self.state) {
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
        let mut future = StateFuture {
            state: unsafe { &*state.get() },
            selector: |state: &i32| {
                if *state == 1 {
                    Some(())
                } else {
                    None
                }
            },
        };
        assert!(block_on(poll_once(&mut future)).is_none());
        *state.get_mut() = 1;
        assert!(block_on(poll_once(&mut future)).is_some());
    }
}

