use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_char;

use libc::size_t;

use super::bindings::root::std::string;

cpp! {{
    #include <string>

    struct struct_with_strings {
        int32_t index;
        std::string first_name;
        std::string last_name;
    };
}}

#[repr(C)]
/// This should be used as only (mutable) references.
/// Small string optimization makes unsafe to move `String` instances.
/// `String::drop` is also prohibited for this reason.
pub struct String(string);

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.c_str() == other.c_str()
    }
}

impl Eq for String {}

impl Drop for String {
    fn drop(&mut self) {
        panic!("Do not drop `String`!");
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c_str().to_string_lossy())
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.c_str().to_string_lossy())
    }
}

impl AsRef<CStr> for String {
    fn as_ref(&self) -> &CStr {
        self.c_str()
    }
}

impl String {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> size_t {
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "const std::string*"] -> size_t as "size_t" {
                return self->size();
            })
        }
    }

    pub fn c_str(&self) -> &CStr {
        #[allow(deprecated)]
        unsafe {
            CStr::from_ptr(cpp!([self as "const std::string*"]
                  -> *const c_char as "const char*" {
                return self->c_str();
            }))
        }
    }

    pub fn assign<S: AsRef<CStr>>(&mut self, s: &S) {
        let s = s.as_ref();
        let ptr = s.as_ptr();
        #[allow(deprecated)]
        unsafe {
            cpp!([self as "std::string*", ptr as "const char*"] {
                self->assign(ptr);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[repr(C)]
    struct StructWithStrings {
        index: i32,
        first_name: String,
        last_name: String,
    }

    #[test]
    fn unittest_struct_with_strings() {
        #[allow(deprecated)]
        let x = unsafe {
            cpp!([] -> &mut StructWithStrings as "struct_with_strings*" {
                static struct_with_strings x{23, "boncheol", "gu"};
                return &x;
            })
        };
        assert_eq!(x.index, 23);
        assert_eq!(x.first_name.c_str().to_string_lossy(), "boncheol");
        assert_eq!(x.last_name.c_str().to_string_lossy(), "gu");

        x.first_name.assign(&CString::new("junmo").unwrap());
        assert_eq!(x.first_name.c_str().to_string_lossy(), "junmo");

        x.last_name.assign(&x.first_name);
    }
}
