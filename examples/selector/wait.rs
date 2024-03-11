use flurx::Scheduler;
use flurx::selector::wait;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<usize>::new();
    scheduler.schedule(|tc| async move {
        println!("*** Start ***");
        tc.task(wait::until(|state| {
            println!("count: {state}");
            state < 10
        })).await;

        tc.task(wait::while_(|state| {
            println!("count: {state}");
            state == 20
        })).await;
        println!("*** Finish ***");
    });

    for i in 0..=20 {
        scheduler.run(i).await;
    }
}