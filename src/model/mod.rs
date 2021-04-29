mod builtin_options;
mod builtin_options_impl;
pub mod stl;

use std::ffi::c_void;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fmt, mem, slice};

use libc::size_t;
use stl::memory::UniquePtr;
use stl::string::String as StlString;
use stl::vector::{
    VectorInsert, VectorOfBool, VectorOfF32, VectorOfI32, VectorOfI64, VectorOfU8,
    VectorOfUniquePtr,
};

pub use crate::bindings::flatbuffers::NativeTable;
pub use crate::bindings::tflite::*;
use crate::{Error, Result};
pub use builtin_options::{
    BuiltinOptionsUnion, ConcatEmbeddingsOptionsT, ReshapeOptionsT, SqueezeOptionsT,
};
pub use builtin_options_impl::*;

#[repr(C)]
#[derive(Debug)]
pub struct QuantizationDetailsUnion {
    pub typ: QuantizationDetails,
    pub value: *mut c_void,
}

impl PartialEq for QuantizationDetailsUnion {
    fn eq(&self, other: &Self) -> bool {
        self.typ == QuantizationDetails::QuantizationDetails_NONE
            && other.typ == QuantizationDetails::QuantizationDetails_NONE
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BufferT {
    _vtable: NativeTable,
    pub data: VectorOfU8,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct QuantizationParametersT {
    _vtable: NativeTable,
    pub min: VectorOfF32,
    pub max: VectorOfF32,
    pub scale: VectorOfF32,
    pub zero_point: VectorOfI64,
    pub details: QuantizationDetailsUnion,
    pub quantized_dimension: i32,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TensorT {
    _vtable: NativeTable,
    pub shape: VectorOfI32,
    pub typ: TensorType,
    pub buffer: u32,
    pub name: StlString,
    pub quantization: UniquePtr<QuantizationParametersT>,
    pub is_variable: bool,
    pub sparsity: UniquePtr<Self>,
    pub shape_signature: VectorOfU8,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct OperatorT {
    _vtable: NativeTable,
    pub opcode_index: u32,
    pub inputs: VectorOfI32,
    pub outputs: VectorOfI32,
    pub builtin_options: BuiltinOptionsUnion,
    pub custom_options: VectorOfU8,
    pub custom_options_format: CustomOptionsFormat,
    pub mutating_variable_inputs: VectorOfBool,
    pub intermediates: VectorOfI32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct OperatorCodeT {
    _vtable: NativeTable,
    pub builtin_code: BuiltinOperator,
    pub custom_code: StlString,
    pub version: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubGraphT {
    _vtable: NativeTable,
    pub tensors: VectorOfUniquePtr<TensorT>,
    pub inputs: VectorOfI32,
    pub outputs: VectorOfI32,
    pub operators: VectorOfUniquePtr<OperatorT>,
    pub name: StlString,
}

#[repr(C)]
#[derive(Debug)]
pub struct MetadataT {
    _vtable: NativeTable,
    pub name: StlString,
    pub buffer: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ModelT {
    _vtable: NativeTable,
    pub version: u32,
    pub operator_codes: VectorOfUniquePtr<OperatorCodeT>,
    pub subgraphs: VectorOfUniquePtr<SubGraphT>,
    pub description: StlString,
    pub buffers: VectorOfUniquePtr<BufferT>,
    pub metadata_buffer: VectorOfI32,
    pub metadata: VectorOfUniquePtr<MetadataT>,
    pub signature_defs: VectorOfU8,
}

impl Clone for BuiltinOptionsUnion {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const BuiltinOptionsUnion*", cloned_ref as "BuiltinOptionsUnion*"] {
                new (cloned_ref) BuiltinOptionsUnion(*self);
            });
        }
        cloned
    }
}

impl Clone for UniquePtr<BufferT> {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::unique_ptr<BufferT>*", cloned_ref as "std::unique_ptr<BufferT>*"] {
                if(*self) {
                    new (cloned_ref) std::unique_ptr<BufferT>(new BufferT(**self));
                }
                else {
                    new (cloned_ref) std::unique_ptr<BufferT>();
                }
            });
        }
        cloned
    }
}

impl Clone for UniquePtr<OperatorCodeT> {
    fn clone(&self) -> Self {
        let mut cloned: UniquePtr<OperatorCodeT> = Default::default();
        cloned.builtin_code = self.builtin_code;
        cloned.custom_code.assign(&self.custom_code);
        cloned.version = self.version;
        cloned
    }
}

impl Clone for UniquePtr<QuantizationParametersT> {
    fn clone(&self) -> Self {
        let mut cloned = unsafe { mem::zeroed() };
        let cloned_ref = &mut cloned;
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::unique_ptr<QuantizationParametersT>*", cloned_ref as "std::unique_ptr<QuantizationParametersT>*"] {
                if(*self) {
                    new (cloned_ref) std::unique_ptr<QuantizationParametersT>(new QuantizationParametersT(**self));
                }
                else {
                    new (cloned_ref) std::unique_ptr<QuantizationParametersT>();
                }
            });
        }
        cloned
    }
}

impl Clone for UniquePtr<TensorT> {
    fn clone(&self) -> Self {
        let mut cloned: UniquePtr<TensorT> = Default::default();
        cloned.shape.assign(self.shape.iter().cloned());
        cloned.typ = self.typ;
        cloned.buffer = self.buffer;
        cloned.name.assign(&self.name);
        cloned.quantization = self.quantization.clone();
        cloned.is_variable = self.is_variable;
        cloned
    }
}

impl Clone for UniquePtr<OperatorT> {
    fn clone(&self) -> Self {
        let mut cloned: UniquePtr<OperatorT> = Default::default();
        cloned.opcode_index = self.opcode_index;
        cloned.inputs.assign(self.inputs.iter().cloned());
        cloned.outputs.assign(self.outputs.iter().cloned());
        cloned.builtin_options = self.builtin_options.clone();
        cloned.custom_options.assign(self.custom_options.iter().cloned());
        cloned.custom_options_format = self.custom_options_format;
        cloned.mutating_variable_inputs = self.mutating_variable_inputs.clone();
        cloned
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct Model(UniquePtr<ModelT>);

impl Clone for Model {
    fn clone(&self) -> Self {
        Self::from_buffer(&self.to_buffer()).unwrap()
    }
}

impl Deref for Model {
    type Target = UniquePtr<ModelT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Model {
    pub fn from_buffer(buffer: &[u8]) -> Option<Self> {
        let len = buffer.len();
        let buffer = buffer.as_ptr();
        let mut model = mem::MaybeUninit::<UniquePtr<ModelT>>::uninit();
        let model_ref = model.as_mut_ptr();
        #[allow(deprecated)]
        let r = unsafe {
            cpp!([buffer as "const void*", len as "size_t", model_ref as "std::unique_ptr<ModelT>*"]
                  -> bool as "bool" {
                auto verifier = flatbuffers::Verifier((const uint8_t *)buffer, len);
                if (!VerifyModelBuffer(verifier)) {
                    return false;
                }

                auto model = tflite::GetModel(buffer)->UnPack();
                new (model_ref) std::unique_ptr<ModelT>(model);
                return true;
            })
        };
        let model = unsafe { model.assume_init() };
        if r && model.is_valid() {
            Some(Self(model))
        } else {
            None
        }
    }

    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        Self::from_buffer(&fs::read(filepath)?)
            .ok_or_else(|| Error::internal_error("failed to unpack the flatbuffer model"))
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let buffer_ptr = &mut buffer;
        let model_ref = &self.0;
        #[allow(deprecated)]
        unsafe {
            cpp!([model_ref as "const std::unique_ptr<ModelT>*", buffer_ptr as "void*"] {
                flatbuffers::FlatBufferBuilder fbb;
                auto model = Model::Pack(fbb, model_ref->get());
                FinishModelBuffer(fbb, model);
                uint8_t* ptr = fbb.GetBufferPointer();
                size_t size = fbb.GetSize();
                rust!(ModelT_to_file [ptr: *const u8 as "const uint8_t*", size: size_t as "size_t", buffer_ptr: &mut Vec<u8> as "void*"] {
                    unsafe { buffer_ptr.extend_from_slice(&slice::from_raw_parts(ptr, size)) };
                });
            })
        }
        buffer
    }

    pub fn to_file<P: AsRef<Path>>(&self, filepath: P) -> Result<()> {
        fs::write(filepath, self.to_buffer())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    use crate::model::stl::vector::{VectorErase, VectorExtract, VectorInsert, VectorSlice};
    use crate::ops::builtin::BuiltinOpResolver;
    use crate::{FlatBufferModel, InterpreterBuilder};

    #[test]
    fn flatbuffer_model_apis_inspect() {
        assert!(Model::from_file("data.mnist10.bin").is_err());

        let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
        assert_eq!(model.version, 3);
        assert_eq!(model.operator_codes.size(), 5);
        assert_eq!(model.subgraphs.size(), 1);
        assert_eq!(model.buffers.size(), 24);
        assert_eq!(model.description.c_str().to_string_lossy(), "TOCO Converted.");

        assert_eq!(
            model.operator_codes[0].builtin_code,
            BuiltinOperator::BuiltinOperator_AVERAGE_POOL_2D
        );

        assert_eq!(
            model.operator_codes.iter().map(|oc| oc.builtin_code).collect::<Vec<_>>(),
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
            subgraph.operators[softmax].builtin_options.typ,
            BuiltinOptions::BuiltinOptions_SoftmaxOptions
        );

        let softmax_options: &SoftmaxOptionsT =
            subgraph.operators[softmax].builtin_options.as_ref();

        #[allow(clippy::float_cmp)]
        {
            assert_eq!(softmax_options.beta, 1.);
        }
    }

    #[test]
    fn flatbuffer_model_apis_mutate() {
        let mut model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
        model.version = 2;
        model.operator_codes.erase(4);
        model.buffers.erase(22);
        model.buffers.erase(23);
        model.description.assign(&CString::new("flatbuffer").unwrap());

        {
            let subgraph = &mut model.subgraphs[0];
            subgraph.inputs.erase(0);
            subgraph.outputs.assign(vec![1, 2, 3, 4]);
        }

        let model_buffer = model.to_buffer();
        let model = Model::from_buffer(&model_buffer).unwrap();
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

    #[test]
    fn flatbuffer_model_apis_insert() {
        let mut model1 = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
        let mut model2 = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();

        let num_buffers = model1.buffers.size();

        let data = model1.buffers[0].data.to_vec();
        let buffer = model1.buffers.extract(0);
        model2.buffers.push_back(buffer);
        assert_eq!(model2.buffers.size(), num_buffers + 1);

        assert_eq!(model2.buffers[num_buffers].data.to_vec(), data);
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn flatbuffer_model_apis_extract() {
        let source_model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
        let source_subgraph = &source_model.subgraphs[0];
        let source_operator = &source_subgraph.operators[0];

        let tensors = source_operator
            .inputs
            .iter()
            .chain(source_operator.outputs.iter())
            .map(|&tensor_index| source_subgraph.tensors[tensor_index as usize].clone());

        let model_buffer = {
            let mut model = Model::default();
            model.version = source_model.version;
            model.description.assign(&source_model.description);
            model.buffers.assign(
                tensors.clone().map(|tensor| source_model.buffers[tensor.buffer as usize].clone()),
            );
            model.operator_codes.push_back(
                source_model.operator_codes[source_operator.opcode_index as usize].clone(),
            );

            let mut subgraph: UniquePtr<SubGraphT> = Default::default();
            subgraph.tensors.assign(tensors);
            for (i, tensor) in subgraph.tensors.iter_mut().enumerate() {
                tensor.buffer = i as u32;
            }
            let mut operator = source_operator.clone();
            operator.opcode_index = 0;
            let num_inputs = operator.inputs.len() as i32;
            let num_outputs = operator.outputs.len() as i32;
            operator.inputs.assign(0..num_inputs);
            operator.outputs.assign(num_inputs..num_inputs + num_outputs);
            subgraph.operators.push_back(operator);
            subgraph
                .inputs
                .assign((0..num_inputs).filter(|&i| model.buffers[i as usize].data.is_empty()));
            subgraph.outputs.assign(num_inputs..num_inputs + num_outputs);
            model.subgraphs.push_back(subgraph);

            let subgraph = &model.subgraphs[0];
            println!("{:?}", subgraph.inputs);
            println!("{:?}", subgraph.outputs);

            for operator in &subgraph.operators {
                println!("{:?}", operator);
            }

            for tensor in &subgraph.tensors {
                println!("{:?}", tensor);
            }

            for buffer in &model.buffers {
                println!("{:?}", buffer);
            }

            for operator_code in &model.operator_codes {
                println!("{:?}", operator_code);
            }
            model.to_buffer()
        };

        let model = Model::from_buffer(&model_buffer).unwrap();
        let subgraph = &model.subgraphs[0];
        let operator = &subgraph.operators[0];
        assert_eq!(model.version, 3);
        assert_eq!(model.description, source_model.description);
        assert_eq!(subgraph.inputs.as_slice(), &[0i32]);
        assert_eq!(subgraph.outputs.as_slice(), &[3i32]);
        assert_eq!(operator.inputs.as_slice(), &[0i32, 1, 2]);
        assert_eq!(operator.outputs.as_slice(), &[3i32]);
        assert_eq!(
            model.operator_codes[operator.opcode_index as usize],
            source_model.operator_codes[source_operator.opcode_index as usize]
        );
        assert_eq!(operator.builtin_options, source_operator.builtin_options);
        assert_eq!(operator.custom_options, source_operator.custom_options);
        assert_eq!(operator.custom_options_format, source_operator.custom_options_format);
        assert_eq!(operator.mutating_variable_inputs, source_operator.mutating_variable_inputs);

        let tensors: Vec<_> = operator
            .inputs
            .iter()
            .chain(operator.outputs.iter())
            .map(|&tensor_index| &subgraph.tensors[tensor_index as usize])
            .collect();

        let source_tensors: Vec<_> = source_operator
            .inputs
            .iter()
            .chain(source_operator.outputs.iter())
            .map(|&tensor_index| &source_subgraph.tensors[tensor_index as usize])
            .collect();

        assert_eq!(tensors.len(), source_tensors.len());
        for (tensor, source_tensor) in tensors.into_iter().zip(source_tensors.into_iter()) {
            assert_eq!(tensor.shape, source_tensor.shape);
            assert_eq!(tensor.typ, source_tensor.typ);
            assert_eq!(tensor.name, source_tensor.name);
            assert_eq!(tensor.quantization, source_tensor.quantization);
            assert_eq!(tensor.is_variable, source_tensor.is_variable);
            assert_eq!(
                model.buffers[tensor.buffer as usize],
                source_model.buffers[source_tensor.buffer as usize]
            );
        }
    }

    #[test]
    fn unittest_buffer_clone() {
        let (buffer1, buffer2) = {
            let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
            let buffer = &model.buffers[0];
            (buffer.clone(), buffer.clone())
        };
        assert_eq!(buffer1.data.as_slice(), buffer2.data.as_slice());
    }

    #[test]
    fn unittest_tensor_clone() {
        let (tensor1, tensor2) = {
            let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
            let tensor = &model.subgraphs[0].tensors[0];
            (tensor.clone(), tensor.clone())
        };

        assert_eq!(tensor1.shape.as_slice(), tensor2.shape.as_slice());
        assert_eq!(tensor1.typ, tensor2.typ);
        assert_eq!(tensor1.buffer, tensor2.buffer);
        assert_eq!(tensor1.name.c_str(), tensor2.name.c_str());
        assert_eq!(tensor1.is_variable, tensor2.is_variable);
    }

    #[test]
    fn unittest_operator_clone() {
        let (operator1, operator2) = {
            let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
            let operator = &model.subgraphs[0].operators[0];
            (operator.clone(), operator.clone())
        };

        assert_eq!(operator1.opcode_index, operator2.opcode_index);
        assert_eq!(operator1.inputs.as_slice(), operator2.inputs.as_slice());
        assert_eq!(operator1.outputs.as_slice(), operator2.outputs.as_slice());
        assert_eq!(operator1.builtin_options.typ, operator2.builtin_options.typ);
        assert_eq!(operator1.custom_options.as_slice(), operator2.custom_options.as_slice());
        assert_eq!(operator1.custom_options_format, operator2.custom_options_format);
        assert_eq!(
            operator1.mutating_variable_inputs.iter().collect::<Vec<_>>(),
            operator2.mutating_variable_inputs.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn unittest_build_model() {
        let mut model = Model::default();
        model.version = 3;
        model.description.assign(&CString::new("model pad").unwrap());

        {
            let mut pad: UniquePtr<OperatorCodeT> = Default::default();
            pad.builtin_code = BuiltinOperator::BuiltinOperator_PAD;
            pad.version = 1;
            model.operator_codes.push_back(pad);
        }

        model.buffers.assign(vec![UniquePtr::<BufferT>::default(); 3]);

        let mut subgraph: UniquePtr<SubGraphT> = Default::default();

        let mut quantization: UniquePtr<QuantizationParametersT> = Default::default();
        quantization.min.push_back(-1.110_645_3);
        quantization.max.push_back(1.274_200_2);
        quantization.scale.push_back(0.009_352_336);
        quantization.zero_point.push_back(119);

        let mut pad: UniquePtr<OperatorT> = Default::default();
        pad.builtin_options = BuiltinOptionsUnion::PadOptions();
        pad.opcode_index = 0;
        pad.inputs.assign(vec![0, 1]);
        pad.outputs.assign(vec![2]);
        subgraph.operators.push_back(pad);
        subgraph.inputs.assign(vec![0]);
        subgraph.outputs.assign(vec![2]);

        let mut tensor: UniquePtr<TensorT> = Default::default();
        tensor.shape.assign(vec![1, 1]);
        tensor.typ = TensorType::TensorType_UINT8;
        tensor.buffer = 0;
        tensor.name.assign(&CString::new("input_tensor").unwrap());
        tensor.quantization = quantization.clone();
        subgraph.tensors.push_back(tensor);

        let mut tensor: UniquePtr<TensorT> = Default::default();
        tensor.shape.assign(vec![2, 2]);
        tensor.typ = TensorType::TensorType_INT32;
        tensor.buffer = 1;
        tensor.name.assign(&CString::new("shape_tensor").unwrap());
        subgraph.tensors.push_back(tensor);
        model.buffers[1].data.assign(vec![0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);

        let mut tensor: UniquePtr<TensorT> = Default::default();
        tensor.shape.assign(vec![2, 2]);
        tensor.typ = TensorType::TensorType_UINT8;
        tensor.buffer = 2;
        tensor.name.assign(&CString::new("output_tensor").unwrap());
        tensor.quantization = quantization;
        subgraph.tensors.push_back(tensor);

        model.subgraphs.push_back(subgraph);

        let builder = InterpreterBuilder::new(
            FlatBufferModel::build_from_model(&model).unwrap(),
            BuiltinOpResolver::default(),
        )
        .unwrap();
        let mut interpreter = builder.build().unwrap();

        interpreter.allocate_tensors().unwrap();
        interpreter.tensor_data_mut(0).unwrap().copy_from_slice(&[0u8]);

        interpreter.invoke().unwrap();
        assert_eq!(interpreter.tensor_data::<u8>(2).unwrap(), &[119u8, 0, 119, 119]);
    }
}
