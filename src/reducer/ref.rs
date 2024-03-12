use core::future::Future;
use core::mem;

use crate::dispatch::Dispatch;
use crate::prelude::ReactiveTask;
use crate::reducer::base::ReducerInner;
use crate::store::Store;

#[repr(transparent)]
pub struct RefReducer<'state, 'future, State>(ReducerInner<'state, 'future, State, &'state State>);

impl<'state, 'future, State> RefReducer<'state, 'future, State>
    where
        'state: 'future,
        State: 'state + 'future
{
    #[inline]
    pub fn new(store: &'state mut Store<State>) -> RefReducer<'state, 'future, State> {
        Self(ReducerInner::new(store))
    }
}

impl<'state, 'future, State> RefReducer<'state, 'future, State>
    where
        'state: 'future,
        State: 'state + 'future + Default
{
    #[inline]
    pub fn schedule<F, Fut>(&mut self, f: F)
        where
            F: FnOnce(ReactiveTask<'state, &'state State>) -> Fut,
            Fut: Future<Output=()> + 'future
    {
        self.0.scheduler.schedule(f);
    }

    #[inline]
    pub async fn dispatch<D>(&mut self, dispatch: D)
        where D: Dispatch<State>
    {
        let state = mem::take(self.0.store.ref_mut());

        *self.0.store.ref_mut() = dispatch.dispatch(state);
        self.0.scheduler.run(self.0.store.read_ref()).await;
    }
}