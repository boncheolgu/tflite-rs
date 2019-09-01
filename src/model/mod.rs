pub mod stl;

use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::slice;

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
