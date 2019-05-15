use std::sync::Arc;

pub trait OpResolver: Send + Sync {
    fn get_resolver_handle(&self) -> &crate::bindings::OpResolver;
}

impl<T: OpResolver> OpResolver for Arc<T> {
    fn get_resolver_handle(&self) -> &crate::bindings::OpResolver {
        self.as_ref().get_resolver_handle()
    }
}

impl<'a, T: OpResolver> OpResolver for &'a T {
    fn get_resolver_handle(&self) -> &crate::bindings::OpResolver {
        (*self).get_resolver_handle()
    }
}
