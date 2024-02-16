use crate::models::permission;

pub trait Authorize {
    fn resolve_permission(&mut self, identifier: &str) -> impl std::future::Future<Output = Vec<permission::Permission>> + Send;
}
