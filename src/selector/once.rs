use core::marker::PhantomData;

use crate::selector::Selector;

/// Create the task that is executed only once.
#[inline]
pub fn run<F, Out, State>(f: F) -> impl Selector<State, Output=Out>
    where
        F: Fn(State) -> Out,
{
    Once(f, PhantomData)
}

struct Once<F, Out, State>(F, PhantomData<(Out, State)>);

impl<F, Out, State> Selector<State> for Once<F, Out, State>
    where
        F: Fn(State) -> Out
{
    type Output = Out;

    fn select(&self, state: State) -> Option<Self::Output> {
        Some(self.0(state))
    }
}


#[cfg(test)]
mod tests {
    use crate::Scheduler;
    use crate::selector::once;
    use crate::tests::result_event;

    #[tokio::test]
    async fn once_run() {
        let mut scheduler = Scheduler::<&'static str>::default();
        let (tx, rx) = result_event();
        scheduler.schedule(|task| async move {
            let output = task.will(once::run(|state: &'static str| {
                state.to_string()
            })).await;
            tx.set(output);
        });

        scheduler.run("hello").await;
        assert_eq!(rx.get(), "hello");
    }
}