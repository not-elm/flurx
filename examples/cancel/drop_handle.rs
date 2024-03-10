use store::Scheduler;
use store::selector::repeat;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<usize>::default();
    
    scheduler.schedule(|task| async move {
        println!("*** Start ***");
        let t1 = task.run(repeat::forever(|state| {
            println!("count: {state}");
        }));
        let t1 = tokio::spawn(t1);
        
        println!("*** Finish ***");
    });

    for _ in 0..10 {
        reducer.dispatch(|state: &usize| {
            *state + 1
        }).await;
    }
}