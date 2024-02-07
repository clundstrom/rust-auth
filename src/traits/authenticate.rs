pub trait Authenticate {
    async fn authenticate(&self, username: &str, password: &str) -> bool;
}
