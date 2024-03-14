pub use own::Reducer;
pub use r#ref::RefReducer;

mod base;
mod own;
mod r#ref;

#[cfg(test)]
mod tests {
    use crate::reducer::Reducer;
    use crate::Scheduler;
    use crate::selector::wait;
    use crate::store::Store;
    use crate::tests::result_event;

    #[tokio::test]
    async fn wait_while_state_reached_2() {
        let mut store = Store::<i32>::default();

        let (r1, r2) = result_event::<bool>();
        let mut scheduler = Scheduler::new();
        scheduler.schedule(|task| async move {
            task.will(wait::until(|state| {
                state == 2
            }))
                .await;
            r1.set(true);
        });
        let mut reducer = Reducer::<i32>::new(&mut store, scheduler);
        reducer.dispatch(|state| {
            state + 1
        }).await;
        reducer.dispatch(|state| {
            state + 1
        }).await;

        assert!(r2.get());
    }
}