use serde::{Deserialize, Serialize};

/// Used to define and describe a `permission` for a `user`.
///
/// ### Arguments
/// * `name` - The name of the permission
/// * `description` - The description of the permission
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Permission {
    name: String,
    description: String,
}
