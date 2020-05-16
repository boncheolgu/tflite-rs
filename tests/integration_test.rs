use std::fs::{self, File};
use std::io::Read;

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result};

fn test_mnist(model: &FlatBufferModel) -> Result<()> {
    let resolver = BuiltinOpResolver::default();

    let builder = InterpreterBuilder::new(model, &resolver)?;
    let mut interpreter = builder.build()?;

    interpreter.allocate_tensors()?;

    let inputs = interpreter.inputs().to_vec();
    assert_eq!(inputs.len(), 1);

    let input_index = inputs[0];

    let outputs = interpreter.outputs().to_vec();
    assert_eq!(outputs.len(), 1);

    let output_index = outputs[0];

    let input_tensor = interpreter.tensor_info(input_index).unwrap();
    assert_eq!(input_tensor.dims, vec![1, 28, 28, 1]);

    let output_tensor = interpreter.tensor_info(output_index).unwrap();
    assert_eq!(output_tensor.dims, vec![1, 10]);

    let mut input_file = File::open("data/mnist10.bin")?;
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
    test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite")?)?;

    let buf = fs::read("data/MNISTnet_uint8_quant.tflite")?;
    test_mnist(&FlatBufferModel::build_from_buffer(buf)?)
}

#[test]
fn mobilenetv2_mnist() -> Result<()> {
    test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_v2_uint8_quant.tflite")?)?;

    let buf = fs::read("data/MNISTnet_v2_uint8_quant.tflite")?;
    test_mnist(&FlatBufferModel::build_from_buffer(buf)?)
}
