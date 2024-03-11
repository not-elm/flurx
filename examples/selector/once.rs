use flurx::prelude::once;
use flurx::Scheduler;

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::<&'static str>::new();

    scheduler.schedule(|tc| async move {
        let api_uri = tc.task(once::run(|city_id| {
            format!("https://weather.tsukumijima.net/api/forecast/city/{city_id}")
        })).await;
        println!("{:?}", api_uri);
        println!("{:?}", reqwest::get(api_uri).await);
    });

    scheduler.run("400040").await;
    loop {
        scheduler.run("333").await;
    }
}