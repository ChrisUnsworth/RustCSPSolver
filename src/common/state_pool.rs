use crate::common::state::State;

pub trait StatePool<T: State> {
    fn copy(&mut self, state: &T) -> T;

    fn return_state(&mut self, state: T);
}