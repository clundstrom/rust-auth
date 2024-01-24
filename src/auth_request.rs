use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AuthRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}
