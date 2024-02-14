use crate::permission;

pub trait Authorize {
    async fn resolve_permission(&mut self, identifier: &str) -> Vec<permission::Permission>;
}
