use std::future::Future;
use std::mem;

use crate::dispatch::Dispatch;
use crate::prelude::TaskCreator;
use crate::reducer::base::ReducerInner;
use crate::store::Store;

#[repr(transparent)]
pub struct RefReducer<'a, 'b, State>(ReducerInner<'a, 'b, State, &'a State>);

impl<'a, 'b, State> RefReducer<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn new(store: &'a mut Store<State>) -> RefReducer<'a, 'b, State> {
        Self(ReducerInner::new(store))
    }
}

impl<'a, 'b, State> RefReducer<'a, 'b, State>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn schedule<F>(&mut self, f: impl FnOnce(TaskCreator<'a, &'a State>) -> F)
        where F: Future<Output=()> + 'b
    {
        self.0.scheduler.schedule(f);
    }

    pub async fn dispatch(&mut self, dispatch: impl Dispatch<State>) {
        let state = mem::take(self.0.store.ref_mut());

        *self.0.store.ref_mut() = dispatch.dispatch(state);
        self.0.scheduler.run(self.0.store.read_ref()).await;
    }
}