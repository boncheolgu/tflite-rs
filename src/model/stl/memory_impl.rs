use std::ops::{Deref, DerefMut};
use std::{fmt, mem};

use crate::model::stl::memory::UniquePtr;

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::OperatorCodeT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<OperatorCodeT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<OperatorCodeT>(new OperatorCodeT);
                return sizeof(OperatorCodeT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::OperatorCodeT>(),
                size,
                "crate::model::OperatorCodeT and OperatorCodeT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::OperatorCodeT> {
    type Target = crate::model::OperatorCodeT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<OperatorCodeT>*"] -> *const crate::model::OperatorCodeT as "const OperatorCodeT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::OperatorCodeT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::OperatorCodeT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::TensorT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<TensorT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<TensorT>(new TensorT);
                return sizeof(TensorT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::TensorT>(),
                size,
                "crate::model::TensorT and TensorT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::TensorT> {
    type Target = crate::model::TensorT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<TensorT>*"] -> *const crate::model::TensorT as "const TensorT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::TensorT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::TensorT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::OperatorT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<OperatorT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<OperatorT>(new OperatorT);
                return sizeof(OperatorT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::OperatorT>(),
                size,
                "crate::model::OperatorT and OperatorT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::OperatorT> {
    type Target = crate::model::OperatorT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<OperatorT>*"] -> *const crate::model::OperatorT as "const OperatorT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::OperatorT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::OperatorT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::SubGraphT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<SubGraphT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<SubGraphT>(new SubGraphT);
                return sizeof(SubGraphT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::SubGraphT>(),
                size,
                "crate::model::SubGraphT and SubGraphT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::SubGraphT> {
    type Target = crate::model::SubGraphT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<SubGraphT>*"] -> *const crate::model::SubGraphT as "const SubGraphT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::SubGraphT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::SubGraphT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::BufferT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<BufferT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<BufferT>(new BufferT);
                return sizeof(BufferT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::BufferT>(),
                size,
                "crate::model::BufferT and BufferT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::BufferT> {
    type Target = crate::model::BufferT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<BufferT>*"] -> *const crate::model::BufferT as "const BufferT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::BufferT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::BufferT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::QuantizationParametersT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<QuantizationParametersT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<QuantizationParametersT>(new QuantizationParametersT);
                return sizeof(QuantizationParametersT);
            });
            debug_assert_eq!(mem::size_of::<crate::model::QuantizationParametersT>(), size, "crate::model::QuantizationParametersT and QuantizationParametersT are not equal sizes");
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::QuantizationParametersT> {
    type Target = crate::model::QuantizationParametersT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<QuantizationParametersT>*"] -> *const crate::model::QuantizationParametersT as "const QuantizationParametersT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::QuantizationParametersT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::QuantizationParametersT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::ModelT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<ModelT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<ModelT>(new ModelT);
                return sizeof(ModelT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::ModelT>(),
                size,
                "crate::model::ModelT and ModelT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::ModelT> {
    type Target = crate::model::ModelT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<ModelT>*"] -> *const crate::model::ModelT as "const ModelT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::ModelT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::ModelT>(t);
        }
    }
}

#[allow(deprecated)]
impl Default for UniquePtr<crate::model::MetadataT> {
    fn default() -> Self {
        let mut this = mem::MaybeUninit::uninit();
        let this_ref = this.as_mut_ptr();
        unsafe {
            let size = cpp!([this_ref as "std::unique_ptr<MetadataT>*"] -> usize as "size_t" {
                new (this_ref) std::unique_ptr<MetadataT>(new MetadataT);
                return sizeof(MetadataT);
            });
            debug_assert_eq!(
                mem::size_of::<crate::model::MetadataT>(),
                size,
                "crate::model::MetadataT and MetadataT are not equal sizes"
            );
            this.assume_init()
        }
    }
}

#[allow(deprecated)]
impl Deref for UniquePtr<crate::model::MetadataT> {
    type Target = crate::model::MetadataT;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = cpp!([self as "const std::unique_ptr<MetadataT>*"] -> *const crate::model::MetadataT as "const MetadataT*" {
                return self->get();
            }) as *const Self::Target;

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
            }) as *mut Self::Target;

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

impl crate::model::MetadataT {
    #[allow(unused)]
    fn transmute() {
        unsafe {
            let t = mem::MaybeUninit::<Self>::uninit();
            mem::transmute::<_, crate::bindings::tflite::MetadataT>(t);
        }
    }
}
