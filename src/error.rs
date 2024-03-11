use thiserror::Error;

pub type FutureResult<Out> = Result<Out, AlreadyDroppedScheduler>;

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
#[error("already dropped scheduler")]
pub struct AlreadyDroppedScheduler;