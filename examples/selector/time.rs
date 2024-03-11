use std::time::Duration;

use flurx::Scheduler;
use flurx::selector::delay;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::default();

    scheduler.schedule(|task| async move {
        println!("*** Delay 3 secs ***");
        task.task(delay::time(Duration::from_secs(3))).await;
        println!("*** Finish ***");
    });

    scheduler.run(()).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    scheduler.run(()).await;
}