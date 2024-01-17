use crate::config::CONFIG;
use crate::permission::Permission;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Claims struct
///
/// ### Arguments
/// * `sub` - The subject of the token
/// * `company` - The company of the token
/// * `exp` - The expiration of the token
/// * `permissions` - The vector of permissions to be encoded in the token
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JWTClaim {
    sub: String,
    company: String,
    exp: usize,
    pub(crate) permissions: Vec<Permission>,
}

/// Create a JWT token with the given user id.
///
/// ### Arguments
/// * `user_id` - The user id to be encoded in the token
pub(crate) fn create_token(user_id: &str) -> Result<String, Error> {
    let expiration_seconds: &u64 = &CONFIG.jwt_expiration_time_seconds;
    let expiration_time = Utc::now() + Duration::seconds(*expiration_seconds as i64);

    let claims = JWTClaim {
        sub: user_id.to_owned(),
        company: "Dippen preb AB".to_owned(), // TODO: move to envvar
        exp: expiration_time.timestamp() as usize,
        permissions: vec![], // TODO: set permissions
    };

    // Get the secret key slice from the configuration
    let secret = &CONFIG.jwt_secret_key;

    // Create a key from the base64 encoded string. The key is used to sign the token payload.
    // If the encoding key fails to be created, the function should return an error string
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::default(), &claims, &encoding_key)
}
