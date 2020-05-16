use std::ptr;

use super::stl::vector::VectorOfI32;
use crate::bindings::flatbuffers::NativeTable;
use crate::bindings::tflite::*;

#[repr(C)]
#[derive(Debug)]
pub struct BuiltinOptionsUnion {
    pub typ: BuiltinOptions,
    pub value: *mut NativeTable,
}

impl Default for BuiltinOptionsUnion {
    fn default() -> Self {
        BuiltinOptionsUnion { typ: BuiltinOptions::BuiltinOptions_NONE, value: ptr::null_mut() }
    }
}

impl Drop for BuiltinOptionsUnion {
    fn drop(&mut self) {
        let ptr = self.value;
        #[allow(deprecated)]
        unsafe {
            cpp!([ptr as "flatbuffers::NativeTable*"] {
                delete ptr;
            });
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct ConcatEmbeddingsOptionsT {
    _vtable: NativeTable,
    pub num_channels: i32,
    pub num_columns_per_channel: VectorOfI32,
    pub embedding_dim_per_channel: VectorOfI32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct ReshapeOptionsT {
    _vtable: NativeTable,
    pub new_shape: VectorOfI32,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct SqueezeOptionsT {
    _vtable: NativeTable,
    pub squeeze_dims: VectorOfI32,
}

impl PartialEq for BuiltinOptionsUnion {
    #[allow(clippy::cognitive_complexity)]
    fn eq(&self, other: &Self) -> bool {
        macro_rules! compare {
            ($e:expr, $t:ty) => {
                if self.typ == $e
                    && other.typ == $e
                    && AsRef::<$t>::as_ref(self) == AsRef::<$t>::as_ref(other)
                {
                    return true;
                }
            };
        }

        if self.typ == BuiltinOptions::BuiltinOptions_NONE
            && other.typ == BuiltinOptions::BuiltinOptions_NONE
        {
            return true;
        }
        compare!(BuiltinOptions::BuiltinOptions_Conv2DOptions, Conv2DOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_DepthwiseConv2DOptions, DepthwiseConv2DOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ConcatEmbeddingsOptions, ConcatEmbeddingsOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LSHProjectionOptions, LSHProjectionOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_Pool2DOptions, Pool2DOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SVDFOptions, SVDFOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_RNNOptions, RNNOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_FullyConnectedOptions, FullyConnectedOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SoftmaxOptions, SoftmaxOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ConcatenationOptions, ConcatenationOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_AddOptions, AddOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_L2NormOptions, L2NormOptionsT);
        compare!(
            BuiltinOptions::BuiltinOptions_LocalResponseNormalizationOptions,
            LocalResponseNormalizationOptionsT
        );
        compare!(BuiltinOptions::BuiltinOptions_LSTMOptions, LSTMOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ResizeBilinearOptions, ResizeBilinearOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_CallOptions, CallOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ReshapeOptions, ReshapeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SkipGramOptions, SkipGramOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SpaceToDepthOptions, SpaceToDepthOptionsT);
        compare!(
            BuiltinOptions::BuiltinOptions_EmbeddingLookupSparseOptions,
            EmbeddingLookupSparseOptionsT
        );
        compare!(BuiltinOptions::BuiltinOptions_MulOptions, MulOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_PadOptions, PadOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_GatherOptions, GatherOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_BatchToSpaceNDOptions, BatchToSpaceNDOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SpaceToBatchNDOptions, SpaceToBatchNDOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_TransposeOptions, TransposeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ReducerOptions, ReducerOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SubOptions, SubOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_DivOptions, DivOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SqueezeOptions, SqueezeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SequenceRNNOptions, SequenceRNNOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_StridedSliceOptions, StridedSliceOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ExpOptions, ExpOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_TopKV2Options, TopKV2OptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SplitOptions, SplitOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LogSoftmaxOptions, LogSoftmaxOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_CastOptions, CastOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_DequantizeOptions, DequantizeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_MaximumMinimumOptions, MaximumMinimumOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ArgMaxOptions, ArgMaxOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LessOptions, LessOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_NegOptions, NegOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_PadV2Options, PadV2OptionsT);
        compare!(BuiltinOptions::BuiltinOptions_GreaterOptions, GreaterOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_GreaterEqualOptions, GreaterEqualOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LessEqualOptions, LessEqualOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SelectOptions, SelectOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SliceOptions, SliceOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_TransposeConvOptions, TransposeConvOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SparseToDenseOptions, SparseToDenseOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_TileOptions, TileOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ExpandDimsOptions, ExpandDimsOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_EqualOptions, EqualOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_NotEqualOptions, NotEqualOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ShapeOptions, ShapeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_PowOptions, PowOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ArgMinOptions, ArgMinOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_FakeQuantOptions, FakeQuantOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_PackOptions, PackOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LogicalOrOptions, LogicalOrOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_OneHotOptions, OneHotOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LogicalAndOptions, LogicalAndOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_LogicalNotOptions, LogicalNotOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_UnpackOptions, UnpackOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_FloorDivOptions, FloorDivOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SquareOptions, SquareOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ZerosLikeOptions, ZerosLikeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_FillOptions, FillOptionsT);
        compare!(
            BuiltinOptions::BuiltinOptions_BidirectionalSequenceLSTMOptions,
            BidirectionalSequenceLSTMOptionsT
        );
        compare!(
            BuiltinOptions::BuiltinOptions_BidirectionalSequenceRNNOptions,
            BidirectionalSequenceRNNOptionsT
        );
        compare!(
            BuiltinOptions::BuiltinOptions_UnidirectionalSequenceLSTMOptions,
            UnidirectionalSequenceLSTMOptionsT
        );
        compare!(BuiltinOptions::BuiltinOptions_FloorModOptions, FloorModOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_RangeOptions, RangeOptionsT);
        compare!(
            BuiltinOptions::BuiltinOptions_ResizeNearestNeighborOptions,
            ResizeNearestNeighborOptionsT
        );
        compare!(BuiltinOptions::BuiltinOptions_LeakyReluOptions, LeakyReluOptionsT);
        compare!(
            BuiltinOptions::BuiltinOptions_SquaredDifferenceOptions,
            SquaredDifferenceOptionsT
        );
        compare!(BuiltinOptions::BuiltinOptions_MirrorPadOptions, MirrorPadOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_AbsOptions, AbsOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_SplitVOptions, SplitVOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_UniqueOptions, UniqueOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ReverseV2Options, ReverseV2OptionsT);
        compare!(BuiltinOptions::BuiltinOptions_AddNOptions, AddNOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_GatherNdOptions, GatherNdOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_CosOptions, CosOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_WhereOptions, WhereOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_RankOptions, RankOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_ReverseSequenceOptions, ReverseSequenceOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_MatrixDiagOptions, MatrixDiagOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_QuantizeOptions, QuantizeOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_MatrixSetDiagOptions, MatrixSetDiagOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_HardSwishOptions, HardSwishOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_IfOptions, IfOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_WhileOptions, WhileOptionsT);
        compare!(BuiltinOptions::BuiltinOptions_DepthToSpaceOptions, DepthToSpaceOptionsT);
        false
    }
}

impl Eq for BuiltinOptionsUnion {}

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
    UniqueOptionsT,
    ReverseV2OptionsT,
    AddNOptionsT,
    GatherNdOptionsT,
    CosOptionsT,
    WhereOptionsT,
    RankOptionsT,
    ReverseSequenceOptionsT,
    MatrixDiagOptionsT,
    QuantizeOptionsT,
    MatrixSetDiagOptionsT,
    HardSwishOptionsT,
    IfOptionsT,
    WhileOptionsT,
    DepthToSpaceOptionsT,
}
