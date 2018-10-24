use std::os::raw::c_void;

pub trait OpResolver {
    fn get_resolver_handle(&self) -> *mut c_void;
}
