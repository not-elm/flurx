pub trait Dispatch<State> {
    /// Dispatch the new state to [`Store`](crate::store::Store).
    ///
    /// This method is used in [`Scheduler::run`](crate::scheduler::Scheduler::run) 
    /// and executes [`Reactors`](crate::scheduler::Reactor) based on the latest state generated by this method.
    ///
    fn dispatch(self, current: State) -> State;
}


impl<State, F> Dispatch<State> for F
    where F: FnOnce(State) -> State
{
    fn dispatch(self, current: State) -> State {
        (self)(current)
    }
}


