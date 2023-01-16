use crate::common::state::State;

trait Variable<V> {

    fn initialise<T: State>(&self, state: &mut T);

    fn is_instantiated<T: State>(&self, state: &T) -> bool;

    fn is_empty<T: State>(&self, state: &T) -> bool;

    fn set_value<T: State>(&self, state: &mut T, value: V) -> bool;

    fn remove_value<T: State>(&self, state: &mut T, value: V) -> bool;

    fn pretty_domain<T: State>(&self, state: &T) -> &str;
}