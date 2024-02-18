use std::pin::Pin;
use serde::{Deserialize, Serialize};
use crate::connectors::ldap::LdapConnector;
use crate::traits::auth::Auth;

/// Enum for available connectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Connector {
    Ldap,
    Dummy,
}
