#![allow(non_camel_case_types)]
use std::marker::PhantomData;
// use crate::{model::Model};

// pub enum Fetch<M: Model> {
//     All,
//     One,
//     Optional,
//     __(PhantomData<M>),
// }
// 
// struct q;
// 
// impl<M: Model, const F: Fetch<M>> FnOnce<()> for q {
//     
// }
// 

struct Struct;

trait Trait<Gen> {}

impl<D, Arg: Trait<D>> FnOnce<(Arg,)> for Struct {
    type Output = D;
    extern "rust-call" fn call_once(self, (arg,): (Arg,)) -> Self::Output {
        
    }
}
