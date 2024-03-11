
pub mod repeat;
pub mod wait;
pub mod delay;
pub mod once;

pub trait Selector<State>{
    type Output;

    fn select(&self, state: State) -> Option<Self::Output>;
}


impl<State, Output, F> Selector<State> for F
    where F: Fn(State) -> Option<Output> + Unpin
{
    type Output = Output;

    fn select(&self, state: State) -> Option<Self::Output> {
        (self)(state)
    }
}

