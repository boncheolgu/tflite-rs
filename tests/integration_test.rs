extern crate failure;

extern crate tflite;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;

use failure::Fallible;

use std::sync::Arc;
use tflite::model::stl::vector::{VectorInsert, VectorRemove, VectorSlice};
use tflite::model::{BuiltinOperator, BuiltinOptions, ModelT, SoftmaxOptionsT};
use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder};

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
    test_mnist(&FlatBufferModel::build_from_buffer(buf).unwrap()).unwrap();
}

#[test]
fn mobilenetv2_mnist() {
    test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_v2_uint8_quant.tflite").unwrap())
        .unwrap();

    let mut f = File::open("data/MNISTnet_v2_uint8_quant.tflite").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    test_mnist(&FlatBufferModel::build_from_buffer(buf).unwrap()).unwrap();
}

#[test]
fn threadsafe_types() {
    fn send_sync<T: Send + Sync>(_t: &T) {}
    let model = FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite")
        .expect("Unable to build flatbuffer model");
    send_sync(&model);
    let resolver = Arc::new(BuiltinOpResolver::default());
    send_sync(&resolver);
    let builder = InterpreterBuilder::new(model, resolver).expect("Not able to build builder");
    send_sync(&builder);
    let interpreter = builder.build().expect("Not able to build model");
    send_sync(&interpreter);
}

#[test]
fn flatbuffer_model_apis_inspect() {
    let model = ModelT::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
    assert_eq!(model.version, 3);
    assert_eq!(model.operator_codes.size(), 5);
    assert_eq!(model.subgraphs.size(), 1);
    assert_eq!(model.buffers.size(), 24);
    assert_eq!(
        model.description.c_str().to_string_lossy(),
        "TOCO Converted."
    );

    assert_eq!(
        model.operator_codes[0].builtin_code,
        BuiltinOperator::BuiltinOperator_AVERAGE_POOL_2D
    );

    assert_eq!(
        model
            .operator_codes
            .iter()
            .map(|oc| oc.builtin_code)
            .collect::<Vec<_>>(),
        vec![
            BuiltinOperator::BuiltinOperator_AVERAGE_POOL_2D,
            BuiltinOperator::BuiltinOperator_CONV_2D,
            BuiltinOperator::BuiltinOperator_DEPTHWISE_CONV_2D,
            BuiltinOperator::BuiltinOperator_SOFTMAX,
            BuiltinOperator::BuiltinOperator_RESHAPE
        ]
    );

    let subgraph = &model.subgraphs[0];
    assert_eq!(subgraph.tensors.size(), 23);
    assert_eq!(subgraph.operators.size(), 9);
    assert_eq!(subgraph.inputs.as_slice(), &[22]);
    assert_eq!(subgraph.outputs.as_slice(), &[21]);

    let softmax = subgraph
        .operators
        .iter()
        .position(|op| {
            model.operator_codes[op.opcode_index as usize].builtin_code
                == BuiltinOperator::BuiltinOperator_SOFTMAX
        })
        .unwrap();

    assert_eq!(subgraph.operators[softmax].inputs.as_slice(), &[4]);
    assert_eq!(subgraph.operators[softmax].outputs.as_slice(), &[21]);
    assert_eq!(
        subgraph.operators[softmax].builtin_options.type_,
        BuiltinOptions::BuiltinOptions_SoftmaxOptions
    );

    let softmax_options: &SoftmaxOptionsT = subgraph.operators[softmax].builtin_options.as_ref();
    assert_eq!(softmax_options.beta, 1.);
}

#[test]
fn flatbuffer_model_apis_mutate() {
    let mut model = ModelT::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
    model.version = 2;
    model.operator_codes.erase(4);
    model.buffers.erase(22);
    model.buffers.erase(23);
    model
        .description
        .assign(CString::new("flatbuffer").unwrap());

    {
        let subgraph = &mut model.subgraphs[0];
        subgraph.inputs.erase(0);
        subgraph.outputs.assign(vec![1, 2, 3, 4]);
    }

    let model_buffer = model.to_buffer();
    let model = ModelT::from_buffer(&model_buffer);
    assert_eq!(model.version, 2);
    assert_eq!(model.operator_codes.size(), 4);
    assert_eq!(model.subgraphs.size(), 1);
    assert_eq!(model.buffers.size(), 22);
    assert_eq!(model.description.c_str().to_string_lossy(), "flatbuffer");

    let subgraph = &model.subgraphs[0];
    assert_eq!(subgraph.tensors.size(), 23);
    assert_eq!(subgraph.operators.size(), 9);
    assert!(subgraph.inputs.as_slice().is_empty());
    assert_eq!(subgraph.outputs.as_slice(), &[1, 2, 3, 4]);
}
