use crate::permission;

pub(crate) trait Authorize {
    async fn resolve_permission(&self, identifier: &str) -> Vec<permission::Permission>;
}
