use bindings;

pub trait OpResolver {
    fn get_resolver_handle(&self) -> *mut bindings::OpResolver;
}
