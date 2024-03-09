use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::selector::StateSelector;

pub struct StateFuture<'a, State, Selector>

{
    selector: Selector,
    state: &'a State,
}


impl<'a, State, Selector> StateFuture<'a, State, Selector>
    where
        Selector: StateSelector<State> + 'a
{
    pub fn new(state: &'a State, selector: Selector) -> impl Future<Output=Selector::Output> + 'a {
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
    fn count() {
        let mut state = UnsafeCell::new(0);

        let mut future = StateFuture::new(unsafe {
            &*(state.get())
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

