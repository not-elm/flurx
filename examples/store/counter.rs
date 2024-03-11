use flurx::reducer::Reducer;
use flurx::selector::wait;
use flurx::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::<usize>::new(&mut store);

    reducer.schedule(|task| async move {
        println!("*** Start ***");
        
        task.task(wait::until(|state| {
            println!("count: {state}");
            state < 10
        }))
            .await;
        println!("*** Finish ***");
    });

    for _ in 0..10 {
        reducer.dispatch(|state| {
            state + 1
        }).await;
    }
}