pub trait Authenticate {
    async fn authenticate(&mut self, username: &str, password: &str) -> bool;
}
