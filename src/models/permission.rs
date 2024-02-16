use crate::models::Access;
use serde::{Deserialize, Serialize};

/// Used to define and describe a `permission`.
///
/// A `permission` is a right or authority given to some entity.
/// The right or authority granted by a `permission` is defined by the `access_type` field.
///
/// ### Arguments
/// * `name` - The name of the permission
/// * `description` - The description of the permission
/// * `access_kind` - The type of access the permission grants.
#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) access_type: Access,
}
