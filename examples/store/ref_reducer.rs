use flurx::reducer::RefReducer;
use flurx::selector::wait;
use flurx::store::Store;

#[tokio::main]
async fn main() {
    let mut store = Store::<String>::default();
    let mut reducer = RefReducer::new(&mut store);

    reducer.schedule(|task| async move {
        println!("*** Start ***");
        task.will(wait::until(|state: &String| {
            println!("state: {state}");
            state.len() < 20
        }))
            .await;
        println!("*** Finish ***");
    });

    for i in 0..=20 {
        reducer.dispatch(|mut state: String| {
            state.push(char::from_digit(i % 10, 10).unwrap());
            state
        }).await;
    }
}