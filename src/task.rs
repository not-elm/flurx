use std::future::Future;

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


    pub fn add<Output, Selector>(&self, selector: Selector) -> StateFuture<'a, State, Selector>
        where Selector: StateSelector<State, Output=Output>
    {
        StateFuture::new(self.state, selector)
    }


    pub fn wait_until(&self, f: impl Fn(&State) -> bool + 'static) -> impl Future<Output=()> + 'a {
        self.add(Until::create(f))
    }

    pub fn wait_while(&self, f: impl Fn(&State) -> bool + 'static) -> impl Future<Output=()> + 'a {
        self.add(While::create(f))
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