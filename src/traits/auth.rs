use crate::traits::{Authenticate, Authorize};

/// Trait for authentication and authorization
/// Combines the `Authenticate` and `Authorize` traits
pub trait Auth: Authenticate + Authorize {}
