#![allow(clippy::transmute_num_to_bytes)]
use std::ops::{Deref, DerefMut};
use std::{fmt, mem};

use crate::model::stl::memory::UniquePtr;

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::OperatorCodeT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<OperatorCodeT>*"] {
                new (this_ref) std::unique_ptr<OperatorCodeT>(new OperatorCodeT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::OperatorCodeT> {
    type Target = crate::model::OperatorCodeT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<OperatorCodeT>*"] -> *const crate::model::OperatorCodeT as "const OperatorCodeT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::OperatorCodeT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<OperatorCodeT>*"] -> *mut crate::model::OperatorCodeT as "OperatorCodeT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::OperatorCodeT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::TensorT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<TensorT>*"] {
                new (this_ref) std::unique_ptr<TensorT>(new TensorT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::TensorT> {
    type Target = crate::model::TensorT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<TensorT>*"] -> *const crate::model::TensorT as "const TensorT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::TensorT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<TensorT>*"] -> *mut crate::model::TensorT as "TensorT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::TensorT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::OperatorT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<OperatorT>*"] {
                new (this_ref) std::unique_ptr<OperatorT>(new OperatorT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::OperatorT> {
    type Target = crate::model::OperatorT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<OperatorT>*"] -> *const crate::model::OperatorT as "const OperatorT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::OperatorT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<OperatorT>*"] -> *mut crate::model::OperatorT as "OperatorT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::OperatorT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::SubGraphT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<SubGraphT>*"] {
                new (this_ref) std::unique_ptr<SubGraphT>(new SubGraphT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::SubGraphT> {
    type Target = crate::model::SubGraphT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<SubGraphT>*"] -> *const crate::model::SubGraphT as "const SubGraphT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::SubGraphT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<SubGraphT>*"] -> *mut crate::model::SubGraphT as "SubGraphT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::SubGraphT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::BufferT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<BufferT>*"] {
                new (this_ref) std::unique_ptr<BufferT>(new BufferT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::BufferT> {
    type Target = crate::model::BufferT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<BufferT>*"] -> *const crate::model::BufferT as "const BufferT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::BufferT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<BufferT>*"] -> *mut crate::model::BufferT as "BufferT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::BufferT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::QuantizationParametersT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<QuantizationParametersT>*"] {
                new (this_ref) std::unique_ptr<QuantizationParametersT>(new QuantizationParametersT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::QuantizationParametersT> {
    type Target = crate::model::QuantizationParametersT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<QuantizationParametersT>*"] -> *const crate::model::QuantizationParametersT as "const QuantizationParametersT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::QuantizationParametersT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<QuantizationParametersT>*"] -> *mut crate::model::QuantizationParametersT as "QuantizationParametersT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::QuantizationParametersT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::ModelT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<ModelT>*"] {
                new (this_ref) std::unique_ptr<ModelT>(new ModelT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::ModelT> {
    type Target = crate::model::ModelT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<ModelT>*"] -> *const crate::model::ModelT as "const ModelT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::ModelT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<ModelT>*"] -> *mut crate::model::ModelT as "ModelT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::ModelT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::MetadataT> {
    fn default() -> Self {
        let mut this: Self = unsafe { mem::zeroed() };
        let this_ref = &mut this;
        unsafe {
            cpp!([this_ref as "std::unique_ptr<MetadataT>*"] {
                new (this_ref) std::unique_ptr<MetadataT>(new MetadataT);
            })
        }
        this
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::MetadataT> {
    type Target = crate::model::MetadataT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<MetadataT>*"] -> *const crate::model::MetadataT as "const MetadataT*" {
                return self->get();
            });

            ptr.as_ref().unwrap()
        }
    }
}

#[allow(deprecated)]
impl DerefMut for UniquePtr<crate::model::MetadataT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = cpp!([self as "std::unique_ptr<MetadataT>*"] -> *mut crate::model::MetadataT as "MetadataT*" {
                return self->get();
            });

            ptr.as_mut().unwrap()
        }
    }
}

#[allow(deprecated)]
impl fmt::Debug for UniquePtr<crate::model::MetadataT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.deref())
    }
}
