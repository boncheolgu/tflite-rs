#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;

mod bindings;
mod error;
mod interpreter;
pub mod model;

pub use error::Error;
pub use interpreter::*;

pub type Result<T> = ::std::result::Result<T, Error>;
