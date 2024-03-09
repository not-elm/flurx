use std::cell::UnsafeCell;

#[derive(Default)]
pub struct Store<State> {
    state: UnsafeCell<State>,
}

impl<State> Store<State> {
    pub(crate) fn ref_mut(&mut self) -> &mut State {
        self.state.get_mut()
    }

    pub(crate) unsafe fn unsafe_ref<'a>(&self) -> &'a State {
        &*self.state.get()
    }
}


impl<State> Store<Option<State>> {
    pub const fn uninit() -> Store<Option<State>> {
        Store {
            state: None
        }
    }
}