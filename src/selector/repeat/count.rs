use core::marker::PhantomData;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::selector::Selector;

/// Create the task that continues to run the specified number of times.
///
/// ## Panics
///
/// Count is 0.
#[inline]
pub fn count<F, State, Out>(count: usize, f: F) -> impl Selector<State, Output=Out> 
    where 
        F: Fn(State) -> Out
{
    assert_ne!(count, 0, "`count` must be greater than or equal to 1.");

    RepeatCount {
        to: count,
        count: AtomicUsize::new(1),
        f,
        _m1: PhantomData,
        _m2: PhantomData,
    }
}

struct RepeatCount<F, Out, State> {
    to: usize,
    count: AtomicUsize,
    f: F,
    _m1: PhantomData<State>,
    _m2: PhantomData<Out>,
}

impl<F, Out, State> Selector<State> for RepeatCount<F, Out, State>
    where F: Fn(State) -> Out
{
    type Output = Out;

    fn select(&self, state: State) -> Option<Self::Output> {
        let output = (self.f)(state);
        if self.count.fetch_add(1, Ordering::Relaxed) < self.to {
            None
        } else {
            Some(output)
        }
    }
}


