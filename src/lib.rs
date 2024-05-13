#![allow(
    clippy::implicit_return,
    clippy::min_ident_chars,
    clippy::missing_docs_in_private_items,
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::pub_with_shorthand,
    clippy::pub_without_shorthand,
    clippy::self_named_module_files,
    clippy::single_call_fn
)]

pub use scheduler::Scheduler;

pub mod selector;
pub mod task;

pub mod scheduler;
pub mod error;
#[cfg(feature = "reducer")]
pub mod store;
#[cfg(feature = "reducer")]
pub mod dispatch;
#[cfg(feature = "reducer")]
pub mod reducer;

pub mod prelude {
    pub use crate::{
        error::{AlreadyDroppedScheduler, FutureResult},
        scheduler::Scheduler,
        selector::*,
        task::ReactiveTask,
    };
    #[cfg(feature = "reducer")]
    pub use crate::dispatch::Dispatch;
    #[cfg(feature = "reducer")]
    pub use crate::reducer::{Reducer, RefReducer};
    #[cfg(feature = "reducer")]
    pub use crate::store::Store;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    pub struct ResultEvent<T>(Arc<Mutex<T>>);


    impl<T> Clone for ResultEvent<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }


    #[allow(unused)]
    pub fn result_event<T: Default>() -> (ResultEvent<T>, ResultEvent<T>) {
        let r1 = ResultEvent(Arc::new(Mutex::new(T::default())));
        let r2 = ResultEvent(Arc::clone(&r1.0));
        (r1, r2)
    }
}
