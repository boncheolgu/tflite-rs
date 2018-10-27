#[macro_use]
extern crate cpp;
#[macro_use]
extern crate failure;
extern crate libc;
extern crate ndarray;

mod bindings;
pub mod context;
pub mod interpreter;
pub mod model;
pub mod op_resolver;
pub mod ops;

pub use interpreter::Interpreter;
pub use model::{FlatBufferModel, InterpreterBuilder};
