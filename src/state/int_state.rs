use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub struct IntState {
    pub data: Vec<u32>,
}

impl State for IntState {
    fn get_int(&self, state_ref: &StateRef) -> i32 {
        i32::from_ne_bytes(self.data[state_ref.idx].to_ne_bytes())
    }

    fn set_int(&mut self, state_ref: &StateRef, value: i32) {
        self.data[state_ref.idx] = u32::from_ne_bytes(value.to_ne_bytes())
    }

    fn copy_to(&self, other: &mut IntState) {
        other.data.copy_from_slice(&self.data);
    }

    fn get_domain(&self, state_ref: &StateRef, size: u32) -> u32 {
        (self.data[state_ref.idx] >> state_ref.offset) & ((1 << size) - 1)
    }

    fn get_domain_max(&self, state_ref: &StateRef, size: u32) -> i32 {
        fast_math::log2_raw(self.get_domain(state_ref, size) as f32) as i32
    }

    fn get_domain_min(&self, state_ref: &StateRef, size: u32) -> i32 {
        self.get_domain(state_ref, size).trailing_zeros() as i32
    }

    fn set_domain(&mut self, state_ref: &StateRef, size: u32, value: u32) {
        self.data[state_ref.idx] &= !(((1 << size) - 1) << state_ref.offset);
        self.data[state_ref.idx] += value << state_ref.offset;
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

    #[test]
    fn get_set_domain() {        
        let sr1: StateRef = StateRef { idx: 0, offset: 0 };
        let sr2: StateRef = StateRef { idx: 0, offset: 4 };
        let sr3: StateRef = StateRef { idx: 0, offset: 8 };

        let mut state: IntState = IntState { data: [ 0 ].to_vec() };

        let val1 = 0b1011;
        let val2 = 0b0101;
        let val3 = 0b0110;

        state.set_domain(&sr1, 4, val1);
        state.set_domain(&sr2, 4, val2);
        state.set_domain(&sr3, 4, val3);

        assert_eq!(state.get_domain(&sr1, 4), val1);
        assert_eq!(state.get_domain(&sr2, 4), val2);
        assert_eq!(state.get_domain(&sr3, 4), val3);

        let val4 = 0b1001;

        state.set_domain(&sr2, 4, val4);

        state.set_domain(&sr1, 4, val1);
        state.set_domain(&sr2, 4, val4);
        state.set_domain(&sr3, 4, val3);
    }

    #[test]
    fn get_domain_min() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let mut state: IntState = IntState { data: [ 0b1111 ].to_vec() };

        assert_eq!(state.get_domain_min(&sr, 4), 0);

        state.set_domain(&sr, 4, 0b1110);

        assert_eq!(state.get_domain_min(&sr, 4), 1);

        state.set_domain(&sr, 4, 0b1100);

        assert_eq!(state.get_domain_min(&sr, 4), 2);

        state.set_domain(&sr, 4, 0b1000);

        assert_eq!(state.get_domain_min(&sr, 4), 3);
    }

    #[test]
    fn get_domain_max() {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let mut state: IntState = IntState { data: [ 0b1111 ].to_vec() };

        assert_eq!(state.get_domain_max(&sr, 4), 3);

        state.set_domain(&sr, 4, 0b0111);

        assert_eq!(state.get_domain_max(&sr, 4), 2);

        state.set_domain(&sr, 4, 0b0011);

        assert_eq!(state.get_domain_max(&sr, 4), 1);

        state.set_domain(&sr, 4, 0b0001);

        assert_eq!(state.get_domain_max(&sr, 4), 0);
    }
}