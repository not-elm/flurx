use std::marker::PhantomData;

#[repr(transparent)]
#[derive(Default)]
pub(crate) struct StatePtr<'a, State>(Vec<Option<State>>, PhantomData<&'a State>);


impl<'a, State> StatePtr<'a, State> {
    pub const fn uninit() -> StatePtr<'a, State> {
        StatePtr(Vec::new(), PhantomData)
    }

    pub fn set(&mut self, state: State) {
        if self.0.is_empty() {
            self.0.push(Some(state));
        } else {
            self.0[0] = Some(state);
        }
    }

    pub(crate) fn state_ref(&mut self) -> &'a Option<State> {
        if self.0.is_empty() {
            self.0.push(None)
        }
        
        unsafe {
            let ptr = self.0.as_ptr();
            if ptr.is_null() {
                &None
            } else {
                &*ptr
            }
        }
    }
}

impl<'a, State> Drop for StatePtr<'a, State> {
    fn drop(&mut self) {
        self.0[0].take();
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