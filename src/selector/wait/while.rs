use std::marker::PhantomData;

use crate::selector::Selector;


/// Create the task that continues to run until the state meets the condition.
pub fn while_<F, State>(f: F) -> impl Selector<State>
    where
        F: Fn(State) -> bool,
{
    While(f, PhantomData)
}

struct While<F, State>(F, PhantomData<State>);

impl<F, State> Selector<State> for While<F, State>
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
    use std::sync::{Arc, Mutex};

    use crate::Scheduler;
    use crate::selector::wait;

    #[tokio::test]
    async fn one_while() {
        let mut scheduler = Scheduler::<i32>::default();
        let result = Arc::new(Mutex::new(false));
        let r = result.clone();

        scheduler.schedule(|task| async move {
            task.task(wait::while_(|state| {
                state == 2
            })).await;
            *r.lock().unwrap() = true;
        });

        scheduler.run(1).await;
        assert!(!*result.lock().unwrap());

        scheduler.run(2).await;
        assert!(*result.lock().unwrap());
    }
}