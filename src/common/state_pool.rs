use crate::common::state::State;

pub trait StatePool<T: State<T>> {
    fn copy(&self, state: &T) -> T;

    fn return_state(&self, state: &T);
}