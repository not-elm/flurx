use std::marker::PhantomData;

use crate::selector::Selector;

pub fn forever<State>(f: impl Fn(&State)) -> impl Selector<State, Output=()> {
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
    where F: Fn(&State)
{
    type Output = ();

    fn select(&self, state: &State) -> Option<Self::Output> {
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
        scheduler.schedule(|tc| async move {
            tokio::spawn(async move {
                let result = tc.try_task(repeat::forever(|state: &ResultEvent<usize>| {
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
        tokio::time::sleep(Duration::from_millis(100)).await;
        let count = rx.get();
        assert!(0 < count);
        assert!(rx2.get());
    }
}