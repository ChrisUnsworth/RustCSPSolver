use crate::common::state_builder::StateBuilder;
use crate::common::state_ref::StateRef;
use crate::state::int_state::IntState;

pub struct IntStateBuilder {
    size: usize,
    map: Vec<u8>
}

impl Default for IntStateBuilder {
    fn default() -> Self {
        Self { 
            size: Default::default(), 
            map: Default::default() 
        }
    }
}

impl IntStateBuilder {
    fn add_small_domain(&mut self, size: u8) -> StateRef {
        let o = self.map.iter().position(|x| x + size <= 32);
        let idx: usize;
        let offset: u8;

        if let Some(i) = o {
            idx = i;
            offset = self.map[idx];
            self.map[idx] += size;
        } else {
            idx = self.size;
            offset = 0;
            self.map.push(size);
            self.size += 1;
        }

        StateRef { idx, offset }
    }

    fn add_large_domain(&mut self, size: u16) -> StateRef {
        let n: usize = (size / 32).into();
        let r: u8 = (size % 32) as u8;
        let state_ref = StateRef { idx: self.size, offset: 0 };
        self.size += n;
        self.map.extend(vec![32; n]);
        if r != 0 {
            self.map.push(r);
            self.size += 1;
        }

        state_ref
    }
}

impl StateBuilder<IntState> for IntStateBuilder {
    fn add_domain(&mut self, size: u16) -> StateRef {
        return if size < 32 {
            self.add_small_domain(size as u8)
        } else {
            self.add_large_domain(size)
        }
    }

    fn add_int(&mut self) -> StateRef {
        let idx: usize = self.size;
        self.map.push(32);
        self.size += 1;
        StateRef { idx, offset: 0 }
    }

    fn add_double(&mut self) -> StateRef {
        let idx: usize = self.size;
        self.map.extend([32, 32]);
        self.size += 2;
        StateRef { idx, offset: 0 }
    }

    fn add_float(&mut self) -> StateRef {
        let idx: usize = self.size;
        self.map.push(32);
        self.size += 1;
        StateRef { idx, offset: 0 }
    }

    fn add_long(&mut self) -> StateRef {
        let idx: usize = self.size;
        self.map.extend([32, 32]);
        self.size += 2;
        StateRef { idx, offset: 0 }
    }

    fn get_state(&self) -> IntState {
        IntState { data: vec![ 0; self.size ] }
    }

    fn get_size(&self) -> usize { 
        self.size 
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ref_equal(sr: StateRef, idx: usize, offset: u8) {
        match sr {
            StateRef { idx: i, offset: o } => assert_eq!((i, o), (idx, offset))
        };
    }

    #[test]
    fn add_two_ints() {
        let mut sb: IntStateBuilder = IntStateBuilder { ..Default::default() };
        let sr1 = sb.add_int();
        assert_ref_equal(sr1, 0, 0);
        let sr2 = sb.add_int();
        assert_ref_equal(sr2, 1, 0);
    }

    #[test]
    fn add_three_small_domains() {
        let mut sb: IntStateBuilder = IntStateBuilder { ..Default::default() };
        let sr1 = sb.add_domain(16);
        assert_ref_equal(sr1, 0, 0);
        let sr2 = sb.add_domain(16);
        assert_ref_equal(sr2, 0, 16);
        let sr3 = sb.add_domain(16);
        assert_ref_equal(sr3, 1, 0);
    }
}