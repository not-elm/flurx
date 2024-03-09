mod future;
pub mod selector;
pub mod scheduler;
pub mod store;
pub mod dispatch;
pub mod reducer;


#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;

    use futures_lite::future::{block_on, poll_once};

    pub(crate) fn poll_once_block<F>(future: &mut Pin<&mut F>) -> Option<F::Output>
        where F: Future
    {
        block_on(poll_once(future))
    }
}
