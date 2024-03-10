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
    use crate::Scheduler;
    use crate::selector::repeat;
    use crate::tests::{result_event, ResultEvent};

    #[tokio::test]
    async fn select_task() {
        let mut scheduler = Scheduler::<ResultEvent<usize>>::default();
        let (tx, rx) = result_event::<usize>();
        scheduler.schedule(|task| async move {
            let t1 = task.task(repeat::forever(|state: &ResultEvent<usize>| {
                state.set(state.get() + 1);
            }));
            let t2 = task.task(repeat::count(1, |_| {}));
            tokio::select! {
                _ = t1 => {},
                _ = t2 => {}
            }
        });
        scheduler.run(tx.clone()).await;
        scheduler.run(tx.clone()).await;

        let count = rx.get();
        assert!(count <= 1);
    }
}