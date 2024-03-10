use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::selector::Selector;

pub fn time<State>(duration: Duration) -> impl Selector<State> {
    Time {
        start: Instant::now(),
        duration,
        _m: PhantomData,
    }
}

struct Time<State> {
    start: Instant,
    duration: Duration,
    _m: PhantomData<State>,
}


impl<State> Selector<State> for Time<State>
{
    type Output = ();

    fn select(&self, _: &State) -> Option<Self::Output> {
        let elapsed = Instant::now().duration_since(self.start);
        if self.duration <= elapsed {
            Some(())
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Scheduler;
    use crate::selector::delay;
    use crate::tests::result_event;

    #[tokio::test]
    async fn delay_300_ms() {
        let mut scheduler = Scheduler::<()>::default();
        let (tx, rx) = result_event();
        scheduler.schedule(|task| async move {
            task.task(delay::time(Duration::from_millis(300))).await;
            tx.set(true);
        });
        scheduler.run(()).await;
        tokio::time::sleep(Duration::from_millis(300)).await;
        scheduler.run(()).await;

        assert!(rx.get());
    }
}