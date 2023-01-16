use crate::common::variables::int_domain_var::IntDomainVar;
use crate::common::variables::int_var::IntVar;
use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub struct IntSmallDomainVar {
    state_ref: StateRef,
    min: i32,
    size: u8
}

impl IntSmallDomainVar {
    fn set_domain<T: State>(&self, state: &mut T, domain: u32) -> bool {
        let old_domain = state.get_domain(&self.state_ref, self.size.into());
        let new_domain = domain & old_domain;
        if old_domain != new_domain {
            state.set_domain(&self.state_ref, self.size.into(), new_domain);
            true
        } else {
            false
        }        
    }

    fn get_domain<T: State>(&self, state: &T) -> u32 {
        state.get_domain(&self.state_ref, self.size.into())
    }
}

impl IntDomainVar for IntSmallDomainVar {
    fn initialise<T: State>(&self, state: &mut T) {
        state.set_domain(&self.state_ref, self.size.into(), (1 << self.size) - 1)
    }

    fn is_instantiated<T: State>(&self, state: &T) -> bool {
        let dom = state.get_domain(&self.state_ref, self.size.into());
        return dom != 0 && (dom & (dom - 1)) == 0;
    }

    fn is_empty<T: State>(&self, state: &T) -> bool {
        state.get_domain(&self.state_ref, self.size.into()) == 0
    }

    fn pretty_domain<T: State>(&self, state: &T) -> String {
        self.enumerate_domain(state)
             .iter()
             .map(|v| v.to_string())
             .collect::<Vec<_>>()
             .join(", ")
    }

    fn get_domain<T: State>(&self, state: &T) -> (Vec<u32>, i32, u32) {
        ([self.get_domain(state)].to_vec(), self.min, self.size.into())
    }

    fn set_domain<T: State>(&self, state: &mut T, domain: Vec<u32>) -> bool {
        self.set_domain(state, domain[0])
    }

    fn domain_minus<T: State>(&self, state: &mut T, domain: Vec<u32>) -> bool {
        self.set_domain(state, !domain[0])
    }

    fn enumerate_domain<T: State>(&self, state: &T) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let dom = state.get_domain(&self.state_ref, self.size.into());
        let mut mask: u32 = 1;

        for i in 0..self.size.into() {
            if (mask & dom) == mask {
                result.push(i + self.min);
            }

            mask <<= 1;
        }

        result
    }
}

impl IntVar for IntSmallDomainVar {
    fn min(&self) -> i32 {
        self.min
    }

    fn size(&self) -> i32 {
        self.size.into()
    }

    fn max(&self) -> i32 {
        self.min + self.size() - 1
    }

    fn get_domain_max<T: State>(&self, state: &T) -> i32 {
        state.get_domain_max(&self.state_ref, self.size.into())
    }

    fn get_domain_min<T: State>(&self, state: &T) -> i32 {
        state.get_domain_min(&self.state_ref, self.size.into())
    }

    fn set_max<T: State>(&self, state: &mut T, max: i32) -> bool {
        if max >= self.size() + self.min {
            false
        } else if max < self.min {
            self.set_domain(state, 0)
        } else {
            let mask: u32 = (1 << (max - self.min + 1)) - 1;
            let old_dom = state.get_domain(&self.state_ref, self.size.into());
            let new_dom = old_dom & mask;
            self.set_domain(state, new_dom)
        }
    }

    fn set_min<T: State>(&self, state: &mut T, min: i32) -> bool {
        if min <= self.min {
            false
        } else if min >= i32::from(self.size) + self.min {
            self.set_domain(state, 0)
        } else {
            let mask: u32 = !((1 << ((min - self.min) as u32)) - 1);
            let old_dom = state.get_domain(&self.state_ref, self.size.into());
            let new_dom = old_dom & mask;
            self.set_domain(state, new_dom)
        }            
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::int_state::IntState;

    fn get_var(min: i32, size: u8) -> (IntSmallDomainVar, IntState)
    {
        let sr: StateRef = StateRef { idx: 0, offset: 0 };
        let var :IntSmallDomainVar = IntSmallDomainVar { state_ref: sr,  min: min, size: size };
        let mut state: IntState = IntState { data: [ 0 ].to_vec() };
        var.initialise(&mut state);
        (var, state)
    }

    #[test]
    fn build_works() {
        let (var, _) = get_var(0, 10);        
        assert_eq!(var.min(), 0);
        assert_eq!(var.max(), 9);
    }

    #[test]
    fn set_min_works() {
        let (var, mut state) = get_var(0, 10);
        assert_eq!(var.min(), 0);
        assert_eq!(var.get_domain_min(&state), 0);
        assert_eq!(var.set_min(&mut state, 1), true);
        assert_eq!(var.set_min(&mut state, 1), false);
        assert_eq!(var.min(), 0);
        assert_eq!(var.get_domain_min(&state), 1);
        assert_eq!(var.set_min(&mut state, 7), true);
        assert_eq!(var.set_min(&mut state, 7), false);
        assert_eq!(var.set_min(&mut state, 5), false);
        assert_eq!(var.min(), 0);
        assert_eq!(var.get_domain_min(&state), 7);
    }

    #[test]
    fn set_max_works() {
        let (var, mut state) = get_var(0, 10);
        assert_eq!(var.max(), 9);
        assert_eq!(var.get_domain_max(&state), 9);
        assert_eq!(var.set_max(&mut state, 8), true);
        assert_eq!(var.set_max(&mut state, 8), false);
        assert_eq!(var.max(), 9);
        assert_eq!(var.get_domain_max(&state), 8);
        assert_eq!(var.set_max(&mut state, 2), true);
        assert_eq!(var.set_max(&mut state, 2), false);
        assert_eq!(var.set_max(&mut state, 5), false);
        assert_eq!(var.max(), 9);
        assert_eq!(var.get_domain_max(&state), 2);
    }

    #[test]
    fn enumerate_domain_works() {        
        let (var, mut state) = get_var(0, 10);
        let mut expected: Vec<i32> = (0..10).collect();
        assert_eq!(var.enumerate_domain(&state), expected);
        var.set_max(&mut state, 8);
        expected = (0..9).collect();
        assert_eq!(var.enumerate_domain(&state), expected);
        var.set_min(&mut state, 4);
        expected = (4..9).collect();
        assert_eq!(var.enumerate_domain(&state), expected);
    }

    #[test]
    fn is_empty_works() {
        let (var, mut state) = get_var(1, 3);
        assert!(!var.is_empty(&state));
        var.set_min(&mut state, 4);
        assert!(var.is_empty(&state));
    }

    #[test]
    fn is_instantiated_works() {
        let (var, mut state) = get_var(1, 3);
        assert!(!var.is_instantiated(&state));
        var.set_min(&mut state, 3);
        assert!(var.is_instantiated(&state));
        var.set_min(&mut state, 4);
        assert!(!var.is_instantiated(&state));
    }
}