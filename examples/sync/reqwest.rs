use flurx::Scheduler;

fn main() {
    let mut scheduler = Scheduler::<()>::new();
    scheduler.schedule(|_| async move {
        let response = reqwest::get("https://doc.rust-lang.org/book").await;
        println!("{response:?}");
    });

    while scheduler.exists_pending_reactors() {
        scheduler.run_sync(());
    }
}