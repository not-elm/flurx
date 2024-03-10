use std::marker::PhantomData;

#[repr(transparent)]
#[derive(Default)]
pub(crate) struct StatePtr<'a, State>([State; 1], PhantomData<&'a State>);


impl<'a, State> StatePtr<'a, State> {
    pub const fn new(state: State) -> StatePtr<'a, State> {
        StatePtr([state], PhantomData)
    }

    pub fn state_ref(&self) -> &'a State {
        unsafe { &*self.0.as_ptr() }
    }

    pub fn set(&mut self, state: State) {
        self.0[0] = state;
    }
}