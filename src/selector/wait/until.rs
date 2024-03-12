use std::marker::PhantomData;

use crate::selector::Selector;

/// Create the task continues to run as long as the state meets the condition.
pub fn until<F, State>(f: F) -> impl Selector<State>
    where
        F: Fn(State) -> bool,
{
    Until(f, PhantomData)
}

struct Until<F, State>(F, PhantomData<State>);

impl<F, State> Selector<State> for Until<F, State>
    where
        F: Fn(State) -> bool
{
    type Output = ();

    fn select(&self, state: State) -> Option<Self::Output> {
        if self.0(state) {
            Some(())
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Scheduler;
    use crate::selector::wait;
    use crate::tests::result_event;

    #[tokio::test]
    async fn until_string_is_hello() {
        let mut scheduler = Scheduler::<&'static str>::default();
        let (tx, rx) = result_event();
        scheduler.schedule(|task| async move {
            task.will(wait::until(|state: &'static str| {
                state == "hello"
            })).await;
            tx.set(true);
        });
        scheduler.run("hello").await;
        scheduler.run("hello").await;
        scheduler.run("end").await;

        assert!(rx.get());
    }
}