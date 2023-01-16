use crate::common::state_ref::StateRef;
use crate::common::state::State;

pub trait StateBuilder<T: State>
{
    fn add_domain(&mut self, size: u16) -> StateRef;

    fn add_int(&mut self) -> StateRef;
    fn add_double(&mut self) -> StateRef;
    fn add_float(&mut self) -> StateRef;
    fn add_long(&mut self) -> StateRef;

    fn get_state(&self) -> T;
    fn get_size(&self) -> usize;
}