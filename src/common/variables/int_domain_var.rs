use crate::common::state::State;

pub trait IntDomainVar {

    fn initialise<T: State>(&self, state: &mut T);

    fn is_instantiated<T: State>(&self, state: &T) -> bool;

    fn is_empty<T: State>(&self, state: &T) -> bool;

    fn pretty_domain<T: State>(&self, state: &T) -> String;

    /// Returns the variable domain values held in the given state, along with the initial variable minimum value and its initial size.
    ///
    /// # Arguments
    ///
    /// * `state` - An object that holds the state of this variable
    fn get_domain<T: State>(&self, state: &T) -> (Vec<u32>, i32, u32);

    fn set_domain<T: State>(&self, state: &mut T, domain: Vec<u32>) -> bool;

    fn domain_minus<T: State>(&self, state: &mut T, domain: Vec<u32>) -> bool;

    fn enumerate_domain<T: State>(&self, state: &T) -> Vec<i32>;
}