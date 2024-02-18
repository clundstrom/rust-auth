pub trait Authenticate {
    fn authenticate(&mut self, username: &str, password: &str) -> impl std::future::Future<Output = bool>;
}
