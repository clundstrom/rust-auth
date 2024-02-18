use std::future::Future;
use std::pin::Pin;

/// Trait for authenticating users
///
/// # Arguments
/// * `self` - A mutable reference to the implementing type
/// * `username` - A string slice that holds the username
/// * `password` - A string slice that holds the password
/// # Returns
/// * A `Future` that resolves to a `bool` indicating if the user is authenticated
/// * The future is pinned and boxed to allow for dynamic dispatch. This is necessary when using
/// the trait object in an asynchronous context.
/// * The future is also `Send` to allow for concurrent execution
/// * The lifetime parameter 'a is used to tie the lifetimes of the self reference and the username
/// and password arguments to the lifetime of the returned future. This is necessary because the
/// future returned by this method borrows self and the username and password arguments.
pub trait Authenticate {
    fn authenticate<'a>(
        &'a mut self,
        username: &'a str,
        password: &'a str,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>;
}
