use crate::bindings::OpResolver as SysOpResolver;
use std::sync::Arc;

pub trait OpResolver: Send + Sync {
    fn get_resolver_handle(&self) -> &SysOpResolver;
}

impl<T: OpResolver> OpResolver for Arc<T> {
    fn get_resolver_handle(&self) -> &SysOpResolver {
        self.as_ref().get_resolver_handle()
    }
}

impl<'a, T: OpResolver> OpResolver for &'a T {
    fn get_resolver_handle(&self) -> &SysOpResolver {
        (*self).get_resolver_handle()
    }
}
