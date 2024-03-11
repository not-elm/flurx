use crate::Scheduler;
use crate::store::Store;

pub(crate) struct ReducerInner<'a, 'b, State, ScheduleState = State> {
    pub(crate) store: &'a mut Store<State>,
    pub(crate) scheduler: Scheduler<'a, 'b, ScheduleState>,
}


impl<'a, 'b, State, ScheduleState> ReducerInner<'a, 'b, State, ScheduleState>
    where
        'a: 'b,
        State: 'a + 'b + Default
{
    pub fn new(store: &'a mut Store<State>) -> ReducerInner<'a, 'b, State, ScheduleState> {
        Self {
            store,
            scheduler: Scheduler::new(),
        }
    }
}