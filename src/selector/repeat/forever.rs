use core::marker::PhantomData;

use crate::selector::Selector;


/// Create the task that continues to run indefinitely.
#[inline]
pub fn forever<F, State>(f: F) -> impl Selector<State, Output=()> 
    where 
        F: Fn(State)
{
    Forever {
        f,
        _m1: PhantomData,
    }
}

struct Forever<F, State> {
    f: F,
    _m1: PhantomData<State>,
}

impl<F, State> Selector<State> for Forever<F, State>
    where F: Fn(State)
{
    type Output = ();

    fn select(&self, state: State) -> Option<Self::Output> {
        (self.f)(state);
        None
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Scheduler;
    use crate::selector::repeat;
    use crate::tests::{result_event, ResultEvent};

    #[tokio::test]
    async fn return_error_already_dropped_scheduler() {
        let mut scheduler = Scheduler::<ResultEvent<usize>>::default();
        let (tx, rx) = result_event::<usize>();
        let (tx2, rx2) = result_event::<bool>();
        scheduler.schedule(|task| async move {
            tokio::spawn(async move {
                let result = task.try_will(repeat::forever(|state: ResultEvent<usize>| {
                    state.set(state.get() + 1);
                }))
                    .await;
                if result.is_err() {
                    tx2.set(true);
                }
            });
        });

        scheduler.run(tx.clone()).await;
        drop(scheduler);
        tokio::time::sleep(Duration::from_millis(300)).await;
        let count = rx.get();
        assert_eq!(0, count);
        assert!(rx2.get());
    }
}