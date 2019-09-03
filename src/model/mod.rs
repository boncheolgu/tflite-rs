pub mod stl;

use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::{fmt, slice};

use failure::Fallible;
use libc::size_t;
use stl::memory::UniquePtr;
use stl::string::String as StlString;
use stl::vector::{Vector, VectorOfBool};

pub use crate::bindings::tflite::*;

#[repr(C)]
#[derive(Debug)]
pub struct QuantizationDetailsUnion {
    pub typ: QuantizationDetails,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferT {
    pub data: Vector<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct QuantizationParametersT {
    pub min: Vector<f32>,
    pub max: Vector<f32>,
    pub scale: Vector<f32>,
    pub zero_point: Vector<i64>,
    pub details: QuantizationDetailsUnion,
}

#[repr(C)]
#[derive(Debug)]
pub struct TensorT {
    pub shape: Vector<i32>,
    pub typ_: TensorType,
    pub buffer: u32,
    pub name: StlString,
    pub quantization: UniquePtr<QuantizationParametersT>,
    pub is_variable: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct BuiltinOptionsUnion {
    pub type_: BuiltinOptions,
    pub value: *mut c_void,
}

macro_rules! add_impl_options {
    ($($t:ty,)*) => ($(
        impl AsRef<$t> for BuiltinOptionsUnion {
            fn as_ref(&self) -> & $t {
                unsafe { (self.value as *const $t).as_ref().unwrap() }
            }
        }

        impl AsMut<$t> for BuiltinOptionsUnion {
            fn as_mut(&mut self) -> &mut $t {
                unsafe { (self.value as *mut $t).as_mut().unwrap() }
            }
        }
    )*)
}

add_impl_options! {
    Conv2DOptionsT,
    DepthwiseConv2DOptionsT,
    ConcatEmbeddingsOptionsT,
    LSHProjectionOptionsT,
    Pool2DOptionsT,
    SVDFOptionsT,
    RNNOptionsT,
    FullyConnectedOptionsT,
    SoftmaxOptionsT,
    ConcatenationOptionsT,
    AddOptionsT,
    L2NormOptionsT,
    LocalResponseNormalizationOptionsT,
    LSTMOptionsT,
    ResizeBilinearOptionsT,
    CallOptionsT,
    ReshapeOptionsT,
    SkipGramOptionsT,
    SpaceToDepthOptionsT,
    EmbeddingLookupSparseOptionsT,
    MulOptionsT,
    PadOptionsT,
    GatherOptionsT,
    BatchToSpaceNDOptionsT,
    SpaceToBatchNDOptionsT,
    TransposeOptionsT,
    ReducerOptionsT,
    SubOptionsT,
    DivOptionsT,
    SqueezeOptionsT,
    SequenceRNNOptionsT,
    StridedSliceOptionsT,
    ExpOptionsT,
    TopKV2OptionsT,
    SplitOptionsT,
    LogSoftmaxOptionsT,
    CastOptionsT,
    DequantizeOptionsT,
    MaximumMinimumOptionsT,
    ArgMaxOptionsT,
    LessOptionsT,
    NegOptionsT,
    PadV2OptionsT,
    GreaterOptionsT,
    GreaterEqualOptionsT,
    LessEqualOptionsT,
    SelectOptionsT,
    SliceOptionsT,
    TransposeConvOptionsT,
    SparseToDenseOptionsT,
    TileOptionsT,
    ExpandDimsOptionsT,
    EqualOptionsT,
    NotEqualOptionsT,
    ShapeOptionsT,
    PowOptionsT,
    ArgMinOptionsT,
    FakeQuantOptionsT,
    PackOptionsT,
    LogicalOrOptionsT,
    OneHotOptionsT,
    LogicalAndOptionsT,
    LogicalNotOptionsT,
    UnpackOptionsT,
    FloorDivOptionsT,
    SquareOptionsT,
    ZerosLikeOptionsT,
    FillOptionsT,
    BidirectionalSequenceLSTMOptionsT,
    BidirectionalSequenceRNNOptionsT,
    UnidirectionalSequenceLSTMOptionsT,
    FloorModOptionsT,
    RangeOptionsT,
    ResizeNearestNeighborOptionsT,
    LeakyReluOptionsT,
    SquaredDifferenceOptionsT,
    MirrorPadOptionsT,
    AbsOptionsT,
    SplitVOptionsT,
}

#[repr(C)]
#[derive(Debug)]
pub struct OperatorT {
    pub opcode_index: u32,
    pub inputs: Vector<i32>,
    pub outputs: Vector<i32>,
    pub builtin_options: BuiltinOptionsUnion,
    pub custom_options: Vector<u8>,
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
    pub tensors: Vector<UniquePtr<TensorT>>,
    pub inputs: Vector<i32>,
    pub outputs: Vector<i32>,
    pub operators: Vector<UniquePtr<OperatorT>>,
    pub name: StlString,
}

#[repr(C)]
#[derive(Debug)]
pub struct ModelT {
    pub version: u32,
    pub operator_codes: Vector<UniquePtr<OperatorCodeT>>,
    pub subgraphs: Vector<UniquePtr<SubGraphT>>,
    pub description: StlString,
    pub buffers: Vector<UniquePtr<BufferT>>,
    pub metadata_buffer: Vector<i32>,
}

pub struct Model {
    ptr: *mut ModelT,
}

unsafe impl Sync for Model {}
unsafe impl Send for Model {}

impl Drop for Model {
    fn drop(&mut self) {
        let model_ptr = self.ptr;
        unsafe {
            cpp!([model_ptr as "ModelT*"] {
                delete model_ptr;
            })
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        let ptr = unsafe {
            cpp!([] -> *mut ModelT as "ModelT*" {
                return new ModelT();
            })
        };
        Self { ptr }
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Self::from_buffer(&self.to_buffer())
    }
}

impl Deref for Model {
    type Target = ModelT;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}

impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Model {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let buffer = buffer.as_ptr();
        let ptr = unsafe {
            cpp!([buffer as "const void*"] -> *mut ModelT as "ModelT*" {
                return tflite::GetModel(buffer)->UnPack();
            })
        };
        Self { ptr }
    }

    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Fallible<Self> {
        let mut buf = Vec::new();
        File::open(filepath.as_ref())?.read_to_end(&mut buf)?;

        Ok(Self::from_buffer(&buf))
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let buffer_ptr = &mut buffer;
        let model_ptr = self.ptr;
        unsafe {
            cpp!([model_ptr as "ModelT*", buffer_ptr as "void*"] {
                flatbuffers::FlatBufferBuilder fbb;
                auto model = Model::Pack(fbb, model_ptr);
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
    use std::ffi::CString;

    use crate::model::stl::vector::{VectorInsert, VectorRemove, VectorSlice};

    #[test]
    fn flatbuffer_model_apis_inspect() {
        let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
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

        let softmax_options: &SoftmaxOptionsT =
            subgraph.operators[softmax].builtin_options.as_ref();
        assert_eq!(softmax_options.beta, 1.);
    }

    #[test]
    fn flatbuffer_model_apis_mutate() {
        let mut model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();
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
        let model = Model::from_buffer(&model_buffer);
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
}
