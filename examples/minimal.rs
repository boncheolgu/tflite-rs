use std::env::args;

use std::sync::Arc;
use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result};

pub fn main() -> Result<()> {
    assert_eq!(args().len(), 2, "minimal <tflite model>");

    let filename = args().nth(1).unwrap();

    let model = Arc::new(FlatBufferModel::build_from_file(filename)?);
    let resolver = Arc::new(BuiltinOpResolver::default());

    let builder = InterpreterBuilder::new(model, resolver)?;
    let mut interpreter = builder.build()?;

    interpreter.allocate_tensors()?;

    println!("=== Pre-invoke Interpreter State ===");
    interpreter.print_state();

    interpreter.invoke()?;

    println!("\n\n=== Post-invoke Interpreter State ===");
    interpreter.print_state();
    Ok(())
}
