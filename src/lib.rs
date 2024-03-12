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
pub mod store;
pub mod dispatch;
pub mod reducer;
pub mod scheduler;
pub mod error;

pub mod prelude {
    pub use crate::{
        dispatch::Dispatch,
        error::{AlreadyDroppedScheduler, FutureResult},
        reducer::Reducer,
        scheduler::Scheduler,
        selector::*,
        task::ReactiveTask,
    };
}

#[cfg(test)]
mod tests {
    use core::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};

    use futures_lite::future::{block_on, poll_once};

    #[derive(Default)]
    pub struct ResultEvent<T>(Arc<Mutex<T>>);


    impl<T> Clone for ResultEvent<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    impl<T> ResultEvent<T> where T: Clone {
        pub fn set(&self, t: T) {
            *self.0.lock().unwrap() = t;
        }

        pub fn get(&self) -> T {
            self.0.lock().unwrap().clone()
        }
    }

    pub fn result_event<T: Default>() -> (ResultEvent<T>, ResultEvent<T>) {
        let r1 = ResultEvent(Arc::new(Mutex::new(T::default())));
        let r2 = ResultEvent(Arc::clone(&r1.0));
        (r1, r2)
    }


    pub(in crate) fn poll_once_block<F>(future: &mut Pin<&mut F>) -> Option<F::Output>
        where F: Future
    {
        block_on(poll_once(future))
    }
}
