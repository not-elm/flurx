mod r#while;
mod until;

pub(crate) use r#while::While;
pub(crate) use until::Until;

pub trait StateSelector<State> {
    type Output;

    fn select(&self, state: &State) -> Option<Self::Output>;
}


impl<State, Output, F> StateSelector<State> for F
    where F: Fn(&State) -> Option<Output> + Unpin
{
    type Output = Output;

    fn select(&self, state: &State) -> Option<Self::Output> {
        (self)(state)
    }
}

