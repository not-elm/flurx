use store::reducer::Reducer;
use store::selector::wait;
use store::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::<usize>::new(&mut store);

    reducer.schedule(|task| async move {
        println!("*** Start ***");
        task.run(wait::until(|state| {
            println!("count: {state}");
            *state < 10
        }))
            .await;
        println!("*** Finish ***");
    });

    for _ in 0..10 {
        reducer.dispatch(|state: &usize| {
            *state + 1
        }).await;
    }
}