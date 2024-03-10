use std::marker::PhantomData;

use crate::selector::StateSelector;

pub struct While<F, State>(F, PhantomData<State>);

impl<F, State> Clone for While<F, State>
    where F: Clone
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}


impl<F, State> While<F, State>
    where
        F: FnOnce(&State) -> bool + Clone,

{
    pub fn create(f: F) -> impl StateSelector<State, Output=()> {
        Self(f, PhantomData)
    }
}

impl<F, State> StateSelector<State> for While<F, State>
    where
        F: FnOnce(&State) -> bool + Clone
{
    type Output = ();

    fn select(self, state: &State) -> Option<Self::Output> {
        if self.0(state) {
            Some(())
        } else {
            None
        }
    }
}

// 
// #[cfg(test)]
// mod tests {
//     use std::cell::UnsafeCell;
// 
//     use futures_lite::pin;
// 
//     use crate::scheduler::Task;
//     use crate::selector::r#while::While;
//     use crate::tests::poll_once_block;
// 
//     #[test]
//     fn one_while() {
//         let mut state = UnsafeCell::new(0);
//         let scheduler = Task::new(unsafe {
//             &(*state.get())
//         });
//         let f = scheduler.add(While::new(|state: &i32| {
//             *state == 2
//         }));
// 
//         pin!(f);
//         assert!(poll_once_block(&mut f).is_none());
// 
//         *state.get_mut() = 1;
//         assert!(poll_once_block(&mut f).is_none());
// 
//         *state.get_mut() = 2;
//         assert!(poll_once_block(&mut f).is_some());
//     }
// }