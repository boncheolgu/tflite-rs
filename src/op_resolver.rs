use bindings;

pub trait OpResolver: Send + Sync {
    fn get_resolver_handle(&self) -> &bindings::OpResolver;
}
