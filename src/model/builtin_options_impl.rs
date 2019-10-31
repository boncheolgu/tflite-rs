
use super::{BuiltinOptions, BuiltinOptionsUnion, NativeTable};

impl BuiltinOptionsUnion {
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
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


