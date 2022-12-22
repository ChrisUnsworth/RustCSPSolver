use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub struct IntState<const COUNT: usize> {
    pub data: [i32; COUNT],
}

impl<const COUNT: usize> State for IntState<COUNT> {
    fn get_int(&self, state: &StateRef) -> i32 {
        self.data[state.idx]
    }

    fn set_int(&mut self, state: &StateRef, value: i32) {
        self.data[state.idx] = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_int_with_state_ref() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let state: IntState<2> = IntState { data: [ 1, 2 ] };
        assert_eq!(state.get_int(&sr), 1);
    }

    #[test]
    fn state_set_int_works() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let mut state: IntState<2> = IntState { data: [ 1, 2 ] };
        assert_eq!(state.get_int(&sr), 1);
        state.set_int(&sr, 4);
        assert_eq!(state.get_int(&sr), 4);
    }
}