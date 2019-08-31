#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;
extern crate cpp_stl;
#[macro_use]
extern crate failure;

mod bindings;
mod interpreter;
pub mod model;

pub use interpreter::*;
