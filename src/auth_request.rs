use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}
