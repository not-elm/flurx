use flurx::prelude::Reducer;
use flurx::selector::wait;
use flurx::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::<usize>::new(&mut store);

    reducer.schedule(|task| async move {
        println!("*** Start ***");
        task.will(wait::until(|state| {
            println!("count: {state}");
            state == 5
        }))
            .await;
        println!("*** Finish ***");
    });

    for _ in 0..=5 {
        reducer.dispatch(|state| {
            state + 1
        }).await;
    }
}