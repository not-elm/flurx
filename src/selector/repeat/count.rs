use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::selector::Selector;

///
/// ## Panics
///
/// Count is 0.
pub fn count<State, Out>(count: usize, f: impl Fn(State) -> Out) -> impl Selector<State, Output=Out> {
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


#[cfg(test)]
mod tests {
    use crate::Scheduler;
    use crate::selector::repeat;
    use crate::tests::result_event;

    #[tokio::test]
    async fn repeat_3_count() {
        let mut scheduler = Scheduler::<&'static str>::default();
        let (tx, rx) = result_event();
        scheduler.schedule(|task| async move {
            let output = task.task(repeat::count(2, |state: &'static str| {
                state.to_string()
            })).await;
            tx.set(output);
        });
        scheduler.run("HELLO").await;
        scheduler.run("TEST").await;

        assert_eq!(rx.get(), "TEST");
    }
}