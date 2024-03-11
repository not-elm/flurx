use flurx::prelude::once;
use flurx::Scheduler;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<&'static str>::new();

    scheduler.schedule(|tc| async move {
        tc.task(once::run(|state|{
            println!("{state}");
        })).await;
    });
    
    scheduler.run("HELLO").await;
}