use crate::common::state::State;

pub trait IntVar {

    fn min(&self) -> i32;

    fn size(&self) -> i32;

    fn max(&self) -> i32;

    fn get_domain_max<T: State>(&self, state: &T) -> i32;

    fn get_domain_min<T: State>(&self, state: &T) -> i32;

    fn set_max<T: State>(&self, state: &mut T, max: i32) -> bool;

    fn set_min<T: State>(&self, state: &mut T, min: i32) -> bool;
}