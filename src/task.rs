use crate::future::StateFuture;
use crate::selector::Selector;

pub(crate) type StateRef<'a, State> = &'a State;

pub struct TaskCreator<'a, State> {
    pub(crate) state: StateRef<'a, State>,
}


impl<'a, State> TaskCreator<'a, State> {
    pub async fn task<Output, Sel>(&self, selector: Sel) -> Output
        where Sel: Selector<State, Output=Output>
    {
        StateFuture {
            state: self.state,
            selector,
        }
            .await
    }
}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::pin;

    use crate::task::TaskCreator;
    use crate::tests::poll_once_block;

    #[test]
    fn once_wait() {
        let mut state = UnsafeCell::new(0);
        let task = TaskCreator {
            state: unsafe { &*state.get() }
        };
        let f = task.task(|state: &i32| {
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