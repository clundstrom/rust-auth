use crate::permission;

pub trait Authorize {
    async fn resolve_permission(&self, identifier: &str) -> Vec<permission::Permission>;
}
