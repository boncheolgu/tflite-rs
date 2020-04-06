use std::ffi::CStr;
use std::fmt;

use crate::bindings;

pub type ElementKind = bindings::TfLiteType;
pub type QuantizationParams = bindings::TfLiteQuantizationParams;

pub trait ElemKindOf {
    fn elem_kind_of() -> ElementKind;
}

impl ElemKindOf for f32 {
    fn elem_kind_of() -> ElementKind {
        bindings::TfLiteType::kTfLiteFloat32
    }
}

impl ElemKindOf for u8 {
    fn elem_kind_of() -> ElementKind {
        bindings::TfLiteType::kTfLiteUInt8
    }
}

impl ElemKindOf for i32 {
    fn elem_kind_of() -> ElementKind {
        bindings::TfLiteType::kTfLiteInt32
    }
}

pub struct TensorInfo {
    pub name: String,
    pub element_kind: ElementKind,
    pub dims: Vec<usize>,
}

impl fmt::Debug for TensorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TensorInfo")
            .field("name", &self.name)
            .field("element_kind", &self.element_kind)
            .field("dims", &self.dims)
            .finish()
    }
}

impl<'a> From<&'a bindings::TfLiteTensor> for TensorInfo {
    fn from(t: &'a bindings::TfLiteTensor) -> Self {
        Self {
            name: unsafe { CStr::from_ptr(t.name) }.to_str().unwrap().to_string(),
            element_kind: t.type_,
            dims: {
                let slice = unsafe {
                    let dims = &*t.dims;
                    dims.data.as_slice(dims.size as usize)
                };
                slice.iter().map(|n| *n as usize).collect()
            },
        }
    }
}
