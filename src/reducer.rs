use std::future::Future;
use std::mem;

use pin_project::pin_project;

use crate::dispatch::Dispatch;
use crate::scheduler::Scheduler;
use crate::store::Store;
use crate::task::Task;

#[pin_project]
pub struct Reducer<'a, 'b, State> {
    store: &'a mut Store<State>,

    scheduler: Scheduler<'a, 'b, State>,
}


impl<'a, 'b, State> Reducer<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn new(store: &'a mut Store<State>) -> Reducer<'a, 'b, State> {
        Self {
            store,
            scheduler: Scheduler::default(),
        }
    }

    pub fn schedule<F>(&mut self, f: impl FnOnce(Task<'a, State>) -> F)
        where F: Future<Output=()> + 'b
    {
        self.scheduler.schedule(f)
    }

    pub async fn dispatch(&mut self, dispatch: impl Dispatch<State>) {
        let state = mem::take(self.store.ref_mut());

        *self.store.ref_mut() = dispatch.dispatch(&state);
        self.scheduler.run(dispatch.dispatch(&state)).await;
    }
}


#[cfg(test)]
mod tests {
    use crate::reducer::Reducer;
    use crate::store::Store;
    use crate::tests::result_event;

    #[tokio::test]
    async fn wait_while_state_reached_2() {
        let mut store = Store::<i32>::default();

        let (r1, r2) = result_event::<bool>();

        let mut reducer = Reducer::<i32>::new(&mut store);
        reducer.schedule(|task| async move {
            task.wait_while(|state| {
                *state == 2
            })
                .await;
            r1.set(true);
        });

        reducer.dispatch(|state: &i32| {
            *state + 1
        }).await;
        reducer.dispatch(|state: &i32| {
            *state + 1
        }).await;

        assert!(r2.get());
    }
}