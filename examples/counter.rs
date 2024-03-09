use store::reducer::Reducer;
use store::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::new(&mut store);
    reducer.schedule(|scheduler| async move {
        scheduler.until(|state| {
            println!("state = {state}");
            *state < 2
        })
            .await;
        println!("done!");
    });

    reducer.dispatch(|state| {
        state + 1
    })
        .await;

    reducer.dispatch(|state| {
        state + 1
    })
        .await;
}