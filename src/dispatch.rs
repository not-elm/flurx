pub trait Dispatch<State> {
    fn dispatch(self, current: State) -> State;
}


impl<State, F> Dispatch<State> for F
    where F: FnOnce(State) -> State
{
    fn dispatch(self, current: State) -> State {
        (self)(current)
    }
}


