use flurx::reducer::Reducer;
use flurx::selector::wait;
use flurx::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::new(&mut store);
    reducer.schedule(|task| async move {
        println!("Wait until count less than 10");
        task.task(wait::until(|state| {
            println!("count: {state}");
            *state < 10
        }))
            .await;

        println!("Wait while count reaches 20");
        task.task(wait::while_(|state| {
            println!("count: {state}");
            *state == 20
        }))
            .await;

        println!("Finish");
    });

    for _ in 0..20 {
        reducer.dispatch(|state| {
            state + 1
        })
            .await;
    }
}