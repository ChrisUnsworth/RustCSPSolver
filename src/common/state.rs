use crate::common::state_ref::StateRef;

pub trait State<T> {
    fn copy_to(&self, other: &mut T);

    fn get_int(&self, idx: &StateRef) -> i32;
    fn set_int(&mut self, idx: &StateRef, value: i32);
    

    //fn get_long(&self, idx: &StateRef) -> i64;
    //fn set_long(&mut self, idx: &StateRef, value: i64);
//
    //fn get_double(&self, idx: &StateRef) -> f64;
    //fn set_double(&mut self, idx: &StateRef, value: f64);
//
    //fn get_float(&self, idx: &StateRef) -> f32;
    //fn set_float(&mut self, idx: &StateRef, value: f32);
//
    //fn get_domain(&self, idx: &StateRef, size: u32) -> u32;
    //fn get_domain_long(&self, idx: &StateRef, size: u32) -> u64;
    //fn get_domain_max(&self, idx: &StateRef, size: u32) -> i32;
    //fn get_domain_max_long(&self, idx: &StateRef, size: u32) -> i32;
    //fn get_large_domain_max(&self, idx: &StateRef, size: u32) -> i32;
    //fn get_domain_min(&self, idx: &StateRef, size: u32) -> i32;
    //fn get_domain_min_long(&self, idx: &StateRef, size: u32) -> i32;
    //fn get_large_domain_min(&self, idx: &StateRef, size: u32) -> i32;
    //fn SetDomain(&mut self, idx: &StateRef, size: u32, value: u32);
    //fn SetDomainLong(&mut self, idx: &StateRef, size: u32, value: u64);
//
    //fn get_large_domain(&self, idx: &StateRef, size: u32) -> [u32];
    //fn set_large_domain(&mut self, idx: &StateRef, size: u32, value: &[u32]);
//
    //fn is_same_as(&self, other: &State) -> bool;
}