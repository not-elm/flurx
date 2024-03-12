use flurx::Scheduler;
use flurx::selector::wait;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<usize>::new();
    scheduler.schedule(|task| async move {
        println!("*** Start ***");
        task.will(wait::until(|state| {
            println!("count: {state}");
            state == 10
        })).await;
        println!("*** Finish ***");
    });

    for i in 0..=10 {
        scheduler.run(i).await;
    }
}