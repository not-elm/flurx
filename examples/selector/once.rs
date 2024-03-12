use flurx::prelude::once;
use flurx::Scheduler;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<&'static str>::new();

    scheduler.schedule(|task| async move {
        task.will(once::run(|state|{
            println!("{state}");
        })).await;
    });
    
    scheduler.run("HELLO").await;
}