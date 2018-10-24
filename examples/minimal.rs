#[macro_use]
extern crate failure;

extern crate tflite;

use std::env::args;

use failure::Fallible;

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};

pub fn main() -> Fallible<()> {
    ensure!(args().len() == 2, "minimal <tflite model>");

    let filename = args().nth(1).unwrap();

    let model = FlatBufferModel::build_from_file(filename)?;
    let resolver = BuiltinOpResolver::new();

    let builder = InterpreterBuilder::new(&model, &resolver)?;
    let mut interpreter = builder.build()?;

    interpreter.allocate_tensors()?;

    println!("=== Pre-invoke Interpreter State ===");
    interpreter.print_state();

    interpreter.invoke()?;

    println!("\n\n=== Post-invoke Interpreter State ===");
    interpreter.print_state();

    Ok(())
}
