use std::time::Duration;

use flurx::Scheduler;
use flurx::selector::delay;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::default();

    scheduler.schedule(|tc| async move {
        println!("*** Delay 3 secs ***");
        tc.task(delay::time(Duration::from_secs(3))).await;
        println!("*** Finish ***");
    });

    scheduler.run(()).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    scheduler.run(()).await;
}