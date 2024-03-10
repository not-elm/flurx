use std::marker::PhantomData;

use crate::selector::StateSelector;



pub struct Until<F, State>(F, PhantomData<State>);


impl<F, State> Clone for Until<F, State>
    where F: Clone
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<F, State> Until<F, State>
    where
        F: FnOnce(&State) -> bool + Clone,

{
    pub fn create(f: F) -> impl StateSelector<State, Output=()> {
        Self(f, PhantomData)
    }
}

impl<F, State> StateSelector<State> for Until<F, State>
    where
        F: FnOnce(&State) -> bool + Clone
{
    type Output = ();

    fn select(self, state: &State) -> Option<Self::Output> {
        if self.0(state) {
            None
        } else {
            Some(())
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use std::cell::UnsafeCell;
// 
//     use futures_lite::pin;
// 
//     use crate::task::Task;
//     use crate::selector::until::Until;
//     use crate::tests::poll_once_block;
// 
//     #[test]
//     fn one_until() {
//         let  state = UnsafeCell::new(Some(0));
//         let scheduler = Task::new(&state);
//         let f = scheduler.add(Until::new(|state: &i32| {
//             *state < 2
//         }));
//         pin!(f);
//         assert!(poll_once_block(&mut f).is_none());
// 
//         unsafe { *state.get() = Some(1) };
//         assert!(poll_once_block(&mut f).is_none());
// 
//         unsafe { *state.get() = Some(2) };
//         assert!(poll_once_block(&mut f).is_some());
//     }
// 
// 
//     #[test]
//     fn two_until() {
//         let state = UnsafeCell::new(Some(0));
//         let task = Task::new(&state);
//         let f1 = task.add(Until::new(|state: &i32| {
//             *state < 1
//         }));
//         let f2 = task.add(Until::new(|state: &i32| {
//             *state < 2
//         }));
//         pin!(f1);
//         pin!(f2);
// 
//         assert!(poll_once_block(&mut f1).is_none());
//         assert!(poll_once_block(&mut f2).is_none());
// 
//         unsafe { *state.get() = Some(1) };
//         assert!(poll_once_block(&mut f1).is_some());
//         assert!(poll_once_block(&mut f2).is_none());
// 
//         unsafe { *state.get() = Some(2) };
//         assert!(poll_once_block(&mut f2).is_some());
//     }
// }