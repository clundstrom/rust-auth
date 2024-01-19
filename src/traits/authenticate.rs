pub(crate) trait Authenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}
