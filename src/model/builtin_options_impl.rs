
use super::{BuiltinOptions, BuiltinOptionsUnion, NativeTable};

impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn Conv2DOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new Conv2DOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_Conv2DOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn DepthwiseConv2DOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new DepthwiseConv2DOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_DepthwiseConv2DOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ConcatEmbeddingsOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ConcatEmbeddingsOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ConcatEmbeddingsOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LSHProjectionOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LSHProjectionOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LSHProjectionOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn Pool2DOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new Pool2DOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_Pool2DOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SVDFOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SVDFOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SVDFOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn RNNOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new RNNOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_RNNOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn FullyConnectedOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new FullyConnectedOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_FullyConnectedOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SoftmaxOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SoftmaxOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SoftmaxOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ConcatenationOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ConcatenationOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ConcatenationOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn AddOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new AddOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_AddOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn L2NormOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new L2NormOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_L2NormOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LocalResponseNormalizationOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LocalResponseNormalizationOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LocalResponseNormalizationOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LSTMOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LSTMOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LSTMOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ResizeBilinearOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ResizeBilinearOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ResizeBilinearOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn CallOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new CallOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_CallOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ReshapeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ReshapeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ReshapeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SkipGramOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SkipGramOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SkipGramOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SpaceToDepthOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SpaceToDepthOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SpaceToDepthOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn EmbeddingLookupSparseOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new EmbeddingLookupSparseOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_EmbeddingLookupSparseOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn MulOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new MulOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_MulOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn PadOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new PadOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_PadOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn GatherOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new GatherOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_GatherOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn BatchToSpaceNDOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new BatchToSpaceNDOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_BatchToSpaceNDOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SpaceToBatchNDOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SpaceToBatchNDOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SpaceToBatchNDOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn TransposeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new TransposeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_TransposeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ReducerOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ReducerOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ReducerOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SubOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SubOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SubOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn DivOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new DivOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_DivOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SqueezeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SqueezeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SqueezeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SequenceRNNOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SequenceRNNOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SequenceRNNOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn StridedSliceOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new StridedSliceOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_StridedSliceOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ExpOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ExpOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ExpOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn TopKV2Options() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new TopKV2OptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_TopKV2Options,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SplitOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SplitOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SplitOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LogSoftmaxOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LogSoftmaxOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LogSoftmaxOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn CastOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new CastOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_CastOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn DequantizeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new DequantizeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_DequantizeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn MaximumMinimumOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new MaximumMinimumOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_MaximumMinimumOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ArgMaxOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ArgMaxOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ArgMaxOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LessOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LessOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LessOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn NegOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new NegOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_NegOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn PadV2Options() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new PadV2OptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_PadV2Options,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn GreaterOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new GreaterOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_GreaterOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn GreaterEqualOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new GreaterEqualOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_GreaterEqualOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LessEqualOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LessEqualOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LessEqualOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SelectOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SelectOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SelectOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SliceOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SliceOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SliceOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn TransposeConvOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new TransposeConvOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_TransposeConvOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SparseToDenseOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SparseToDenseOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SparseToDenseOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn TileOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new TileOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_TileOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ExpandDimsOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ExpandDimsOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ExpandDimsOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn EqualOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new EqualOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_EqualOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn NotEqualOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new NotEqualOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_NotEqualOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ShapeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ShapeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ShapeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn PowOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new PowOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_PowOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ArgMinOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ArgMinOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ArgMinOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn FakeQuantOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new FakeQuantOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_FakeQuantOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn PackOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new PackOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_PackOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LogicalOrOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LogicalOrOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LogicalOrOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn OneHotOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new OneHotOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_OneHotOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LogicalAndOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LogicalAndOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LogicalAndOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LogicalNotOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LogicalNotOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LogicalNotOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn UnpackOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new UnpackOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_UnpackOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn FloorDivOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new FloorDivOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_FloorDivOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SquareOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SquareOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SquareOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ZerosLikeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ZerosLikeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ZerosLikeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn FillOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new FillOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_FillOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn BidirectionalSequenceLSTMOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new BidirectionalSequenceLSTMOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_BidirectionalSequenceLSTMOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn BidirectionalSequenceRNNOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new BidirectionalSequenceRNNOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_BidirectionalSequenceRNNOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn UnidirectionalSequenceLSTMOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new UnidirectionalSequenceLSTMOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_UnidirectionalSequenceLSTMOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn FloorModOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new FloorModOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_FloorModOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn RangeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new RangeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_RangeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ResizeNearestNeighborOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ResizeNearestNeighborOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ResizeNearestNeighborOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn LeakyReluOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new LeakyReluOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_LeakyReluOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SquaredDifferenceOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SquaredDifferenceOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SquaredDifferenceOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn MirrorPadOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new MirrorPadOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_MirrorPadOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn AbsOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new AbsOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_AbsOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn SplitVOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new SplitVOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_SplitVOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn UniqueOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new UniqueOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_UniqueOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ReverseV2Options() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ReverseV2OptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ReverseV2Options,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn AddNOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new AddNOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_AddNOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn GatherNdOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new GatherNdOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_GatherNdOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn CosOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new CosOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_CosOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn WhereOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new WhereOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_WhereOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn RankOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new RankOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_RankOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn ReverseSequenceOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new ReverseSequenceOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_ReverseSequenceOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn MatrixDiagOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new MatrixDiagOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_MatrixDiagOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn QuantizeOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new QuantizeOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_QuantizeOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn MatrixSetDiagOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new MatrixSetDiagOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_MatrixSetDiagOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn HardSwishOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new HardSwishOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_HardSwishOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn IfOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new IfOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_IfOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn WhileOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new WhileOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_WhileOptions,
            value,
        }
    }
}


impl BuiltinOptionsUnion {
    #[allow(non_snake_case, deprecated)]
    pub fn DepthToSpaceOptions() -> Self {
        let value = unsafe {
            cpp!([] -> *mut NativeTable as "flatbuffers::NativeTable*" {
                return new DepthToSpaceOptionsT;
            })
        };

        Self {
            typ: BuiltinOptions::BuiltinOptions_DepthToSpaceOptions,
            value,
        }
    }
}


