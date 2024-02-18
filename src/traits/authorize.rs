use std::future::Future;
use std::pin::Pin;
use crate::models::{Permission};

pub trait Authorize {

    /// Resolve permissions for a given identifier
    ///
    /// Lifetime parameter 'a. This parameter is used to tie the lifetimes of
    /// the self reference and the identifier argument to the lifetime of the returned future.
    /// This is necessary because the future returned by this method borrows self and
    /// the identifier argument.
    ///
    /// # Arguments
    /// Mutably borrows `self` and a string slice `identifier`
    ///
    /// # Returns
    /// A `Future` that resolves to a `Vec` of `Permission` objects
    ///
    fn resolve_permission<'a>(&'a mut self, identifier: &'a str) -> Pin<Box<dyn Future<Output = Vec<Permission>> + Send + 'a>>;
}
