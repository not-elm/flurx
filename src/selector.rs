mod r#while;
mod until;
mod once;

pub(crate) use r#while::While;
pub(crate) use until::Until;

pub trait StateSelector<State>: Clone {
    type Output;

    fn select(self, state: &State) -> Option<Self::Output>;
}


impl<State, Output, F> StateSelector<State> for F
    where F: FnOnce(&State) -> Option<Output> + Unpin + Clone
{
    type Output = Output;

    fn select(self, state: &State) -> Option<Self::Output> {
        (self)(state)
    }
}

