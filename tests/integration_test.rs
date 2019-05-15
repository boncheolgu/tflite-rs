extern crate failure;

extern crate tflite;

use std::fs::File;
use std::io::Read;

use failure::Fallible;

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};
use std::sync::Arc;

fn test_mnist(model: &FlatBufferModel) -> Fallible<()> {
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

    let input_tensor = interpreter.tensor_info(input_index)?;
    assert_eq!(input_tensor.dims, vec![1, 28, 28, 1]);

    let output_tensor = interpreter.tensor_info(output_index)?;
    assert_eq!(output_tensor.dims, vec![1, 10]);

    let mut input_file = File::open("data/mnist10.bin")?;
    for i in 0..10 {
        input_file.read_exact(interpreter.tensor_data_mut(input_index)?)?;

        interpreter.invoke()?;

        let output: &[u8] = interpreter.tensor_data(output_index)?;
        let guess = output
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .0;

        println!("{}: {:?}", i, output);
        assert_eq!(i, guess);
    }
    Ok(())
}

#[test]
fn mobilenetv1_mnist() {
    test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite").unwrap())
        .unwrap();

    let mut f = File::open("data/MNISTnet_uint8_quant.tflite").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    test_mnist(&FlatBufferModel::build_from_buffer(&buf).unwrap()).unwrap();
}

#[test]
fn mobilenetv2_mnist() {
    test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_v2_uint8_quant.tflite").unwrap())
        .unwrap();

    let mut f = File::open("data/MNISTnet_v2_uint8_quant.tflite").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    test_mnist(&FlatBufferModel::build_from_buffer(&buf).unwrap()).unwrap();
}

#[test]
fn threadsafe_types() {
    fn send_sync<T: Send + Sync>(_t: &T) {}
    let model = FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite").expect("Unable to build flatbuffer model");
    send_sync(&model);
    let resolver = Arc::new(BuiltinOpResolver::default());
    send_sync(&resolver);
    let builder = InterpreterBuilder::new(model, resolver).expect("Not able to build builder");
    send_sync(&builder);
    let interpreter = builder.build().expect("Not able to build model");
    send_sync(&interpreter);
}
