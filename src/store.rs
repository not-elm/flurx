use std::cell::UnsafeCell;
use std::ptr;

#[derive(Default)]
pub struct Store<State> {
    state: UnsafeCell<State>,
}

impl<State> Store<State> {
    pub(crate) fn read(&self) -> State {
        unsafe { ptr::read(self.state.get()) }
    }

    pub(crate) fn ref_mut(&mut self) -> &mut State {
        self.state.get_mut()
    }
}


impl<State> Store<State> {
    pub const fn new(state: State) -> Store<State> {
        Store {
            state: UnsafeCell::new(state)
        }
    }
}


impl<State> Store<Option<State>> {
    pub const fn uninit() -> Store<Option<State>> {
        Store {
            state: UnsafeCell::new(None)
        }
    }
}