#![recursion_limit = "128"]

#[macro_use]
extern crate cpp;
extern crate cpp_stl;
#[macro_use]
extern crate failure;

mod bindings;
pub mod context;
pub mod interpreter;
pub mod model;
pub mod op_resolver;
pub mod ops;
pub mod schema;

pub use interpreter::Interpreter;
pub use model::{FlatBufferModel, InterpreterBuilder};
