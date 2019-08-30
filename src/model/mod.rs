use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::slice;

use cpp_stl::memory::UniquePtr;
use cpp_stl::string::String as StlString;
use cpp_stl::vector::{
    VectorOfBool, VectorOfF32, VectorOfI32, VectorOfI64, VectorOfU8, VectorOfUniquePtr,
};
use failure::Fallible;
use libc::size_t;

use crate::bindings::{
    BuiltinOperator, BuiltinOptionsUnion, CustomOptionsFormat, QuantizationDetails, TensorType,
};

cpp! {{
    #include "tensorflow/lite/schema/schema_generated.h"
    #include "flatbuffers/flatbuffers.h"

    using OperatorPred = bool (*)(const tflite::OperatorT&);

    #include <cstdio>
    #include <memory>
    using namespace std;
}}

#[repr(C)]
#[derive(Debug)]
pub struct QuantizationDetailsUnion {
    pub typ: QuantizationDetails,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferT {
    pub data: VectorOfU8,
}

#[repr(C)]
#[derive(Debug)]
pub struct QuantizationParametersT {
    pub min: VectorOfF32,
    pub max: VectorOfF32,
    pub scale: VectorOfF32,
    pub zero_point: VectorOfI64,
    pub details: QuantizationDetailsUnion,
}

#[repr(C)]
#[derive(Debug)]
pub struct TensorT {
    pub shape: VectorOfI32,
    pub typ_: TensorType,
    pub buffer: u32,
    pub name: StlString,
    pub quantization: UniquePtr<QuantizationParametersT>,
    pub is_variable: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct OperatorT {
    pub opcode_index: u32,
    pub inputs: VectorOfI32,
    pub outputs: VectorOfI32,
    pub builtin_options: BuiltinOptionsUnion,
    pub custom_options: VectorOfU8,
    pub custom_options_format: CustomOptionsFormat,
    pub mutating_variable_inputs: VectorOfBool,
}

#[repr(C)]
#[derive(Debug)]
pub struct OperatorCodeT {
    pub builtin_code: BuiltinOperator,
    pub custom_code: StlString,
    pub version: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubGraphT {
    pub tensors: VectorOfUniquePtr<TensorT>,
    pub inputs: VectorOfI32,
    pub outputs: VectorOfI32,
    pub operators: VectorOfUniquePtr<OperatorT>,
    pub name: StlString,
}

impl SubGraphT {
    pub fn remove_tensor(&mut self, tensor_index: usize) {
        unsafe {
            cpp!([self as "SubGraphT*", tensor_index as "size_t"] {
                self->tensors.erase(self->tensors.begin() + tensor_index);
            });
        }
    }

    pub fn retain_tensors(&mut self, pred: fn(usize, &TensorT) -> bool) {
        let removed: Vec<_> = self
            .tensors
            .iter()
            .enumerate()
            .filter_map(|(i, op)| if pred(i, op) { None } else { Some(i) })
            .collect();

        for i in removed.into_iter().rev() {
            self.remove_tensor(i);
        }
    }

    pub fn remove_operator(&mut self, operator_index: usize) {
        unsafe {
            cpp!([self as "SubGraphT*", operator_index as "size_t"] {
                self->operators.erase(self->operators.begin() + operator_index);
            });
        }
    }

    pub fn retain_operators(&mut self, pred: fn(usize, &OperatorT) -> bool) {
        let removed: Vec<_> = self
            .operators
            .iter()
            .enumerate()
            .filter_map(|(i, op)| if pred(i, op) { None } else { Some(i) })
            .collect();

        for i in removed.into_iter().rev() {
            self.remove_operator(i);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ModelT {
    pub version: u32,
    pub operator_codes: VectorOfUniquePtr<OperatorCodeT>,
    pub subgraphs: VectorOfUniquePtr<SubGraphT>,
    pub description: StlString,
    pub buffers: VectorOfUniquePtr<BufferT>,
    pub metadata_buffer: VectorOfI32,
}

impl ModelT {
    pub fn from_buffer(buffer: &[u8]) -> Box<Self> {
        let buffer = buffer.as_ptr();
        unsafe {
            Box::from_raw(cpp!([buffer as "const void*"] -> *mut ModelT as "ModelT*" {
                auto model = tflite::GetModel(buffer)->UnPack();
                return model;
            }))
        }
    }

    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Fallible<Box<Self>> {
        let mut buf = Vec::new();
        File::open(filepath.as_ref())?.read_to_end(&mut buf)?;

        Ok(Self::from_buffer(&buf))
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let buffer_ptr = &mut buffer;
        unsafe {
            cpp!([self as "const ModelT*", buffer_ptr as "void*"] {
                flatbuffers::FlatBufferBuilder fbb;
                auto model = Model::Pack(fbb, self);
                FinishModelBuffer(fbb, model);
                uint8_t* ptr = fbb.GetBufferPointer();
                size_t size = fbb.GetSize();
                rust!(ModelT_to_file [ptr: *const u8 as "const uint8_t*", size: size_t as "size_t", buffer_ptr: &mut Vec<u8> as "void*"] {
                    buffer_ptr.extend_from_slice(&slice::from_raw_parts(ptr, size));
                });
            })
        }
        buffer
    }

    pub fn to_file<P: AsRef<Path>>(&self, filepath: P) -> Fallible<()> {
        File::create(filepath.as_ref())?.write_all(&mut self.to_buffer())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpp_stl::vector::VectorSlice;
    use failure::Fallible;

    use crate::bindings;

    #[test]
    fn unittest_layout() {
        use std::mem::{align_of, size_of};

        assert_eq!(size_of::<ModelT>(), size_of::<bindings::ModelT>());
        assert_eq!(align_of::<ModelT>(), align_of::<bindings::ModelT>());

        assert_eq!(size_of::<SubGraphT>(), size_of::<bindings::SubGraphT>());
        assert_eq!(align_of::<SubGraphT>(), align_of::<bindings::SubGraphT>());

        assert_eq!(
            size_of::<OperatorCodeT>(),
            size_of::<bindings::OperatorCodeT>()
        );
        assert_eq!(
            align_of::<OperatorCodeT>(),
            align_of::<bindings::OperatorCodeT>()
        );
    }

    #[test]
    fn unittest_tflite_models() -> Fallible<()> {
        let mut model = ModelT::from_file("data/MNISTnet_uint8_quant.tflite")?;
        assert_eq!(model.version, 3);
        assert_eq!(model.operator_codes.size(), 5);
        dbg!(&model);
        assert_eq!(model.subgraphs.size(), 1);
        let subgraph = &mut model.subgraphs[0];
        assert_eq!(subgraph.operators.size(), 9);
        assert_eq!(subgraph.inputs[0], subgraph.operators[0].inputs[0]);

        subgraph.outputs[0] = 4;

        let buffer = model.to_buffer();
        let model = ModelT::from_buffer(&buffer);
        assert_eq!(model.version, 3);
        assert_eq!(model.operator_codes.size(), 5);
        dbg!(&model);
        assert_eq!(model.subgraphs.size(), 1);
        assert_eq!(model.subgraphs[0].outputs[0], 4);
        Ok(())
    }
}
