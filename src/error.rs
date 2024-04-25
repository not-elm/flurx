use std::fmt::{Display, Formatter};

pub type FutureResult<Out> = Result<Out, AlreadyDroppedScheduler>;

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub struct AlreadyDroppedScheduler;

impl Display for AlreadyDroppedScheduler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("already dropped scheduler")
    }
}

impl std::error::Error for AlreadyDroppedScheduler {}