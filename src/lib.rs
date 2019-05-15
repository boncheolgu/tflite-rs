#[macro_use]
extern crate cpp;
#[macro_use]
extern crate failure;
extern crate libc;

mod bindings;
pub mod context;
pub mod interpreter;
pub mod model;
pub mod op_resolver;
pub mod ops;

pub use interpreter::Interpreter;
pub use model::{FlatBufferModel, InterpreterBuilder};

#[test]
#[should_panic]
fn threadsafe_types() {
    fn send_sync<T: Send + Sync>(_t: &T) {}
    let model = FlatBufferModel::default();
    send_sync(&model);
    let resolver = ops::builtin::resolver::Resolver::default();
    send_sync(&resolver);
    let builder = InterpreterBuilder::<'static>::new(&model, &resolver).expect("Not able to build builder");
    send_sync(&builder);
    let interpreter = builder.build().expect("Not able to build model");
    send_sync(&interpreter);
}
