use flurx::Scheduler;
use flurx::selector::wait;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<usize>::new();
    scheduler.schedule(|task| async move {
        println!("*** Start ***");
        task.task(wait::until(|state| {
            println!("count: {state}");
            state < 10
        })).await;

        task.task(wait::while_(|state| {
            println!("count: {state}");
            state == 20
        })).await;
        println!("*** Finish ***");
    });

    for i in 0..=20 {
        scheduler.run(i).await;
    }
}