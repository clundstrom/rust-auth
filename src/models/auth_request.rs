use serde::Deserialize;
use crate::connectors::Connector;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub connector: Connector,
}
