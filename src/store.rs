use std::cell::UnsafeCell;

pub trait LoadStore<State> {
    fn load(&self) -> &State;
}


#[derive(Default)]
pub struct Store<State> {
    state: UnsafeCell<State>,
}

impl<State> Store<State> {
    pub(crate) fn ref_mut(&mut self) -> &mut State {
        self.state.get_mut()
    }

}


impl<State> Store<Option<State>> {
    pub const fn uninit() -> Store<Option<State>> {
        Store {
            state: UnsafeCell::new(None)
        }
    }
}