use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub struct IntState {
    pub data: Vec<u32>,
}

impl State for IntState {
    fn get_int(&self, state: &StateRef) -> i32 {
        i32::from_ne_bytes(self.data[state.idx].to_ne_bytes())
    }

    fn set_int(&mut self, state: &StateRef, value: i32) {
        self.data[state.idx] = u32::from_ne_bytes(value.to_ne_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_int_with_state_ref() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let state: IntState = IntState { data: [ 1, 2 ].to_vec() };
        assert_eq!(state.get_int(&sr), 1);
    }

    #[test]
    fn state_set_int_works() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let mut state: IntState = IntState { data: [ 1, 2 ].to_vec() };
        
        assert_eq!(state.get_int(&sr), 1);
        state.set_int(&sr, 4);
        assert_eq!(state.get_int(&sr), 4);
    }
}