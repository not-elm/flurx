pub trait Dispatch<State> {
    fn dispatch(&self, current: &State) -> State;
}


impl<State, F> Dispatch<State> for F
    where F: Fn(&State) -> State
{
    fn dispatch(&self, current: &State) -> State {
        (self)(current)
    }
}


