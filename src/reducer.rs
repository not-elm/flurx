use std::future::Future;
use std::pin::Pin;

use crate::store::Store;

type PinFuture<'b> = Pin<Box<dyn Future<Output=()> + 'b>>;


pub struct Reducer<'a, State> {
    store: &'a mut Store<State>,
    futures: Vec<PinFuture<'a>>,
}


impl<'a, State> Reducer<'a, State>
    where State: Default + 'static
{
    pub fn new(store: &'a mut Store<State>) -> Reducer<'a, State> {
        Self {
            store,
            futures: Vec::new(),
        }
    }


    // pub async fn dispatch(&mut self, dispatch: impl Dispatch<State>) {
    //     let state = mem::take(self.store.ref_mut());
    //     *self.store.ref_mut() = dispatch.dispatch(state);
    //     let len = self.futures.len();
    // 
    //     DispatchHandle {
    //         futures: &mut self.futures,
    //         polled: Vec::with_capacity(len),
    //     }
    //         .await
    // }
}



