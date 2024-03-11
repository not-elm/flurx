# flurx

`flurx` is a library that incorporates an asynchronous flow called `Reactor` into the `flux` data flow.

## Examples

<details>

<summary>example reducer</summary>

```rust
#[tokio::main]
async fn main() {
    let mut store = Store::<usize>::default();
    let mut reducer = Reducer::<usize>::new(&mut store);

    reducer.schedule(|tc| async move {
        println!("*** Start ***");

        tc.task(wait::until(|state| {
            println!("count: {state}");
            state < 10
        }))
            .await;
        println!("*** Finish ***");
    });

    for _ in 0..10 {
        reducer.dispatch(|state| {
            state + 1
        }).await;
    }
}
```

</details>


Scheduler is included in Reducer, but it can also be used alone. 
In that case, you can separate it from state management and use only Reactor's functionality.

<details>

<summary>example scheduler</summary>

```rust
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
```

</details>

# License

This crate is licensed under the MIT License or the Apache License 2.0.
