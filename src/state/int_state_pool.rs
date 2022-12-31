use crate::common::state::State;
use crate::common::state_pool::StatePool;
use crate::state::int_state::IntState;

pub struct IntStatePool {
    data: Vec<IntState>,
}

impl IntStatePool {
    fn new() -> IntStatePool {
        IntStatePool { data: Vec::new() }
    }
}

impl StatePool<IntState> for IntStatePool {
    fn copy(&mut self, state: &IntState) -> IntState {
        let mut new_state: IntState = match self.data.pop() {
            Some(s) => s,
            None => IntState { data: vec![0; state.data.len()] },
        };

        state.copy_to(&mut new_state);
        new_state
    }

    fn return_state(&mut self, state: IntState) {
        self.data.push(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::state_ref::StateRef;

    #[test]
    fn copy_works_when_empty() {
        let sr1: StateRef = StateRef { idx: 0, offset: 0 };
        let sr2: StateRef = StateRef { idx: 1, offset: 0 };
        let s1: IntState = IntState { data: [ 1, 2 ].to_vec() };
        let mut pool = IntStatePool::new();
        let s2 = pool.copy(&s1);

        assert_eq!(s2.get_int(&sr1), 1);
        assert_eq!(s2.get_int(&sr2), 2);
    }

    #[test]
    fn copy_works_when_not_empty() {
        let sr1: StateRef = StateRef { idx: 0, offset: 0 };
        let sr2: StateRef = StateRef { idx: 1, offset: 0 };
        let s1: IntState = IntState { data: [ 1, 2 ].to_vec() };
        let s2: IntState = IntState { data: [ 3, 4 ].to_vec() };
        let mut pool = IntStatePool::new();
        pool.return_state(s1);
        let s3 = pool.copy(&s2);

        assert_eq!(s3.get_int(&sr1), 3);
        assert_eq!(s3.get_int(&sr2), 4);
    }
}