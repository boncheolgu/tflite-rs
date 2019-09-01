#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;
#[macro_use]
extern crate failure;

mod bindings;
mod interpreter;
pub mod model;

pub use interpreter::*;
