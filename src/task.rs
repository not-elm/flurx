use crate::future::StateFuture;
use crate::selector::{StateSelector, Until, While};

pub(crate) type StateRef<'a, State> = &'a State;

pub struct Task<'a, State> {
    pub(crate) state: StateRef<'a, State>,
}


impl<'a, State> Task<'a, State> {
    pub(crate) fn new(state: StateRef<'a, State>) -> Task<'a, State> {
        Self {
            state
        }
    }


    pub async fn add<Output, Selector>(&self, selector: Selector) -> Output
        where Selector: StateSelector<State, Output=Output>
    {
        StateFuture::new(self.state, selector).await
    }

    pub async fn once<Output>(&self, f: impl FnOnce(&State) -> Output + Clone + Unpin) -> Output{
        self.add(move |state: &State| {
            Some(f(state))
        }).await
    }

    pub async fn wait_until(&self, f: impl FnOnce(&State) -> bool + Clone) {
        self.add(Until::create(f)).await
    }

    pub async fn wait_while(&self, f: impl FnOnce(&State) -> bool + Clone) {
        self.add(While::create(f)).await
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
        let task = Task::new(unsafe {
            &*state.get()
        });
        let f = task.add(|state: &i32| {
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