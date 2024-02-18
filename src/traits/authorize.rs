use crate::models::{Permission};

pub trait Authorize {
    fn resolve_permission(&mut self, identifier: &str) -> impl std::future::Future<Output = Vec<Permission>>;
}
