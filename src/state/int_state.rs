use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub struct IntState {
    pub data: Vec<u32>,
}

impl State<IntState> for IntState {
    fn get_int(&self, state: &StateRef) -> i32 {
        i32::from_ne_bytes(self.data[state.idx].to_ne_bytes())
    }

    fn set_int(&mut self, state: &StateRef, value: i32) {
        self.data[state.idx] = u32::from_ne_bytes(value.to_ne_bytes())
    }

    fn copy_to(&self, other: &mut IntState) {
        other.data.copy_from_slice(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_to_works() {
        let sr1: StateRef = StateRef { idx: 0, offset: 0 };
        let sr2: StateRef = StateRef { idx: 1, offset: 0 };
        let state1: IntState = IntState { data: [ 1, 2 ].to_vec() };
        let mut state2: IntState = IntState { data: [ 3, 4 ].to_vec() };
        assert_eq!(state2.get_int(&sr1), 3);
        assert_eq!(state2.get_int(&sr2), 4);

        state1.copy_to(&mut state2);
        
        assert_eq!(state2.get_int(&sr1), 1);
        assert_eq!(state2.get_int(&sr2), 2);

    }

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