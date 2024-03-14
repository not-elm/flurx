use core::future::Future;
use core::mem;

use crate::dispatch::Dispatch;
use crate::prelude::ReactiveTask;
use crate::reducer::base::ReducerInner;
use crate::Scheduler;
use crate::store::Store;

#[repr(transparent)]
pub struct Reducer<'state, 'future, State>(ReducerInner<'state, 'future, State, State>);

impl<'state, 'future, State> Reducer<'state, 'future, State>
    where
        'state: 'future,
        State: Clone + 'state + 'future + Default
{
    #[inline]
    pub fn new(store: &'state mut Store<State>, scheduler: Scheduler<'state, 'future, State>) -> Reducer<'state, 'future, State> {
        Self(ReducerInner::new(store, scheduler))
    }
}

impl<'state, 'future, State> Reducer<'state, 'future, State>
    where
        'state: 'future,
        State: Clone + Default + 'state + 'future
{
    #[inline]
    pub fn schedule<F, Fut>(&mut self, f: F)
        where
            F: FnOnce(ReactiveTask<'state, State>) -> Fut,
            Fut: Future<Output=()> + 'future
    {
        self.0.scheduler.schedule(f);
    }

    #[inline]
    pub async fn dispatch<D>(&mut self, dispatch: D)
        where
            D: Dispatch<State>
    {
        let state = mem::take(self.0.store.ref_mut());
        let new_state = dispatch.dispatch(state);
        *self.0.store.ref_mut() = new_state.clone();
        self.0.scheduler.run(new_state).await;
    }

    #[inline]
    #[cfg(feature = "sync")]
    pub fn dispatch_sync<D>(&mut self, dispatch: D)
        where
            D: Dispatch<State>
    {
        use async_compat::CompatExt;
        pollster::block_on(self.dispatch(dispatch).compat());
    }
}