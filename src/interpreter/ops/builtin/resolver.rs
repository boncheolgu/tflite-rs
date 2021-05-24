use crate::interpreter::op_resolver::OpResolver;

pub struct Resolver {
    handle: cxx::UniquePtr<crate::interpreter::cxx::BuiltinOpResolver>,
}

impl OpResolver for Resolver {
    fn get_resolver_handle(&self) -> &crate::interpreter::cxx::OpResolver {
        crate::interpreter::cxx::builtin_op_resolver_to_op_resolver(&self.handle)
    }
}

unsafe impl Send for Resolver {}
unsafe impl Sync for Resolver {}

impl Default for Resolver {
    #[allow(clippy::forget_copy, deprecated)]
    fn default() -> Self {
        Self { handle: crate::interpreter::cxx::builtin_op_resolver() }
    }
}
