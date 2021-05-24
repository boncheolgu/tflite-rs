use std::io::Read;

use std::sync::Arc;
use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result};

fn test_mnist(model: Arc<FlatBufferModel>) -> Result<()> {
    let resolver = Arc::new(BuiltinOpResolver::default());

    let builder = InterpreterBuilder::new(model, resolver)?;
    let mut interpreter = builder.build()?;

    interpreter.allocate_tensors()?;

    let inputs = interpreter.inputs().to_vec();
    assert_eq!(inputs.len(), 1);

    let input_index = inputs[0];

    let outputs = interpreter.outputs().to_vec();
    assert_eq!(outputs.len(), 1);

    let output_index = outputs[0];

    // Todo: implement this tensor_info
    // let input_tensor = interpreter.tensor_info(input_index).unwrap();
    // assert_eq!(input_tensor.dims, vec![1, 28, 28, 1]);
    //
    // let output_tensor = interpreter.tensor_info(output_index).unwrap();
    // assert_eq!(output_tensor.dims, vec![1, 10]);

    // include for testing on other architectures
    let mut input_file = std::io::Cursor::new(include_bytes!("../data/mnist10.bin").as_ref());
    for i in 0..10 {
        input_file.read_exact(interpreter.tensor_data_mut(input_index)?)?;

        interpreter.invoke()?;

        let output: &[u8] = interpreter.tensor_data(output_index)?;
        let guess = output.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;

        println!("{}: {:?}", i, output);
        assert_eq!(i, guess);
    }
    Ok(())
}

#[test]
fn mobilenetv1_mnist() -> Result<()> {
    // include for testing on other architectures
    let buf = include_bytes!("../data/MNISTnet_uint8_quant.tflite");
    test_mnist(Arc::new(FlatBufferModel::build_from_buffer(buf.as_ref())?))
}

#[test]
fn mobilenetv2_mnist() -> Result<()> {
    // include for testing on other architectures
    let buf = include_bytes!("../data/MNISTnet_v2_uint8_quant.tflite");
    test_mnist(Arc::new(FlatBufferModel::build_from_buffer(buf.as_ref())?))
}
