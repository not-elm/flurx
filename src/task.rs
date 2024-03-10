use crate::future::StateFuture;
use crate::selector::Selector;

pub(crate) type StateRef<'a, State> = &'a State;

pub struct Task<'a, State> {
    pub(crate) state: StateRef<'a, State>,
}


impl<'a, State> Task<'a, State> {
    pub async fn run<Output, Sel>(&self, selector: Sel) -> Output
        where Sel: Selector<State, Output=Output>
    {
        StateFuture {
            state: self.state,
            selector,
        }
            .await
    }

    pub async fn once<Output>(&self, f: impl Fn(&State) -> Output + Clone + Unpin) -> Output {
        self.run(move |state: &State| {
            Some(f(state))
        }).await
    }
}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::pin;

    use crate::task::Task;
    use crate::tests::poll_once_block;

    #[test]
    fn once_wait() {
        let mut state = UnsafeCell::new(0);
        let task = Task {
            state: unsafe { &*state.get() }
        };
        let f = task.run(|state: &i32| {
            if *state == 1 {
                Some(())
            } else {
                None
            }
        });
        pin!(f);
        assert!(poll_once_block(&mut f).is_none());
        *state.get_mut() = 1;
        assert!(poll_once_block(&mut f).is_some());
    }
}