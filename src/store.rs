use core::cell::UnsafeCell;

#[derive(Default)]
pub struct Store<State> {
    state: UnsafeCell<State>,
}

impl<State> Store<State> {
    pub(in crate) fn read_ref<'state>(&self) -> &'state State {
        // SAFETY:
        // Lifetime is safe because it coincides with the borrowing period of this `Store`.
        unsafe { &*self.state.get() }
    }

    pub(in crate) fn ref_mut(&mut self) -> &mut State {
        self.state.get_mut()
    }
}

impl<State> Store<State> {
    #[inline]
    pub const fn new(state: State) -> Store<State> {
        Store {
            state: UnsafeCell::new(state)
        }
    }
}

impl<State> Store<Option<State>> {
    #[inline]
    pub const fn uninit() -> Store<Option<State>> {
        Store {
            state: UnsafeCell::new(None)
        }
    }
}