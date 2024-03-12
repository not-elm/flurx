use crate::Scheduler;
use crate::store::Store;

pub(in crate) struct ReducerInner<'state, 'future, State, ScheduleState = State> {
    pub(in crate) store: &'state mut Store<State>,
    pub(in crate) scheduler: Scheduler<'state, 'future, ScheduleState>,
}


impl<'state, 'future, State, ScheduleState> ReducerInner<'state, 'future, State, ScheduleState>
    where
        'state: 'future,
        State: 'state + 'future,
        ScheduleState: Clone + 'state + 'future
{
    pub fn new(store: &'state mut Store<State>) -> ReducerInner<'state, 'future, State, ScheduleState> {
        Self {
            store,
            scheduler: Scheduler::new(),
        }
    }
}