use core::marker::PhantomData;

#[repr(transparent)]
#[derive(Default)]
pub(crate) struct StatePtr<'state, State>(Vec<Option<State>>, PhantomData<&'state State>);


impl<'state, State> StatePtr<'state, State> {
    pub(super) const fn uninit() -> StatePtr<'state, State> {
        StatePtr(Vec::new(), PhantomData)
    }

    #[inline]
    pub fn set(&mut self, state: State) {
        if let Some(now) = self.0.get_mut(0) {
            *now = Some(state);
        } else {
            self.0.push(Some(state));
        }
    }

    pub(in crate) fn state_ref(&mut self) -> &'state Option<State> {
        if self.0.is_empty() {
            self.0.push(None);
        }
        
        // SAFETY:
        // Lifetime can be longer than the actual validity period.
        // In such cases, the content of Option will be None, and panic will occur when the task uses this value.
        unsafe {
            let ptr = self.0.as_ptr();
            &*ptr
        }
    }
}

impl<'state, State> Drop for StatePtr<'state, State> {
    fn drop(&mut self) {
        if let Some(state) = self.0.get_mut(0){
            state.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scheduler::state_ptr::StatePtr;

    struct A;

    #[tokio::test]
    async fn state_ref_come_be_none_after_dropped() {
        let mut ptr = StatePtr::<A>::uninit();
        ptr.set(A);
        let refer = ptr.state_ref();

        assert!(refer.is_some());
        drop(ptr);
        assert!(refer.is_none());
    }
}