pub trait OpResolver: Send + Sync {
    fn get_resolver_handle(&self) -> &super::cxx::OpResolver;
}
