use flurx::prelude::Reducer;
use flurx::Scheduler;
use flurx::selector::wait;
use flurx::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut scheduler  = Scheduler::new();
    scheduler.schedule(|task| async move {
        println!("*** Start ***");
        task.will(wait::until(|state| {
            println!("count: {state}");
            state == 5
        }))
            .await;
        println!("*** Finish ***");
    });
    let mut reducer = Reducer::<usize>::new(&mut store, scheduler);

    for _ in 0..=5 {
        reducer.dispatch(|state| {
            state + 1
        }).await;
    }
}