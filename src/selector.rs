pub mod once;
pub mod repeat;
pub mod wait;
pub mod delay;

pub trait Selector<State>{
    type Output;

    fn select(&self, state: &State) -> Option<Self::Output>;
}


impl<State, Output, F> Selector<State> for F
    where F: Fn(&State) -> Option<Output> + Unpin + Clone
{
    type Output = Output;

    fn select(&self, state: &State) -> Option<Self::Output> {
        (self)(state)
    }
}

