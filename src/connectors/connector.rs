use serde::{Deserialize, Serialize};

/// Enum for available connectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Connector {
    Ldap,
    Dummy,
}
