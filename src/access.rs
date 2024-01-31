use serde::{Deserialize, Serialize};

/// The `AccessKind` enum represents the different types of access an entity can have in the system.
///
/// There are three types of access:
/// * `READ`: The user can read data.
/// * `WRITE`: The user can write or modify data.
/// * `EXECUTE`: The user can execute certain actions.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Access {
    READ,
    WRITE,
    EXECUTE,
}
