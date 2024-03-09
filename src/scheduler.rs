use std::future::Future;
use std::process::Output;

use crate::future::StateFuture;
use crate::selector::{StateSelector, Until};

pub struct Scheduler<State> {
    state: Option<State>,
}


impl<'a, State> Scheduler<'a, State>
    where State: 'static
{
    pub fn new(state: &'a State) -> Scheduler<'a, State> {
        Self {
            state
        }
    }

    pub fn add<Output, Selector>(&self, selector: Selector) -> impl Future<Output=Selector::Output> + 'a
        where Selector: StateSelector<State, Output=Output> + 'a
    {
        StateFuture::new(self.state, selector)
    }
    
    
    pub fn until(&self, f: impl Fn(&State) -> bool + 'static) -> impl Future<Output = ()> + 'a{
       self.add(Until::new(f))
    }
}


#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use futures_lite::pin;

    use crate::scheduler::Scheduler;
    use crate::tests::poll_once_block;

    #[test]
    fn once_wait() {
        let mut state = UnsafeCell::new(1);
        let scheduler = Scheduler::new(unsafe {
            &(*state.get())
        });
        let f = scheduler.add(|state: &i32| {
            if *state == 2 {
                Some(())
            } else {
                None
            }
        });
        pin!(f);
        assert!(poll_once_block(&mut f).is_none());
        *state.get_mut() = 2;
        assert!(poll_once_block(&mut f).is_some());
    }
}