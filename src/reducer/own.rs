use std::future::Future;
use std::mem;

use crate::dispatch::Dispatch;
use crate::prelude::ReactiveTask;
use crate::reducer::base::ReducerInner;
use crate::store::Store;

#[repr(transparent)]
pub struct Reducer<'a, 'b, State>(ReducerInner<'a, 'b, State, State>);

impl<'a, 'b, State> Reducer<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn new(store: &'a mut Store<State>) -> Reducer<'a, 'b, State> {
        Self(ReducerInner::new(store))
    }
}

impl<'a, 'b, State> Reducer<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn schedule<F>(&mut self, f: impl FnOnce(ReactiveTask<'a, State>) -> F)
        where F: Future<Output=()> + 'b
    {
        self.0.scheduler.schedule(f);
    }

    pub async fn dispatch(&mut self, dispatch: impl Dispatch<State>) {
        let state = mem::take(self.0.store.ref_mut());

        *self.0.store.ref_mut() = dispatch.dispatch(state);
        self.0.scheduler.run(self.0.store.read()).await;
    }
}