use crate::config::CONFIG;
use crate::models::Permission;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Claims struct
///
/// ### Arguments
/// * `sub` - The subject of the token
/// * `company` - The company of the token
/// * `exp` - The expiration of the token
/// * `permissions` - The vector of permissions to be encoded in the token
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaim {
    sub: String,
    company: String,
    exp: usize,
    pub(crate) permissions: Vec<Permission>,
}

/// This function is used to create a JWT token for a given user id.
///
/// # Arguments
///
/// * `user_id` - A string slice that holds the user id.
///
/// # Returns
///
/// * `Result<String, Error>` - This function returns a Result. If the token is successfully created,
/// it returns the token as a String. If there is an error during the creation of the token, it returns the error.
///
/// # Example
///
/// ```
/// let token = issue_token("user123", vec![]);
/// match token {
///     Ok(t) => println!("Token: {}", t),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn issue_token(user_id: &str, permissions: Vec<Permission>) -> Result<String, Error> {
    // The expiration time for the token is retrieved from the configuration.
    let expiration_seconds: &u64 = &CONFIG.jwt_expiration_time_seconds;
    // The expiration time is calculated by adding the expiration seconds to the current time.
    let expiration_time = Utc::now() + Duration::seconds(*expiration_seconds as i64);

    let claims = JWTClaim {
        sub: user_id.to_owned(),
        company: CONFIG.jwt_company.clone(),
        exp: expiration_time.timestamp() as usize,
        permissions,
    };


    log::debug!("Issue: {:?}", claims);
    // The secret key for the JWT token is retrieved from the configuration.
    let secret = &CONFIG.jwt_secret_key;

    // The secret key is used to create an encoding key for the JWT token.
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    // The JWT token is encoded with the default header, the created claims, and the encoding key.
    let issued_token = encode(&Header::default(), &claims, &encoding_key);
    log::debug!("Issued token: {:?}", issued_token);
    issued_token
}

/// Validates the provided JWT token.
///
/// # Arguments
///
/// * `token_str` - The JWT token as a String.
///
/// # Returns
///
/// * `Result<(), Error>` - Ok if the token is valid, Err otherwise.
pub async fn validate_token(
    token_str: String,
) -> jsonwebtoken::errors::Result<TokenData<JWTClaim>> {
    decode::<JWTClaim>(
        &token_str,
        &DecodingKey::from_secret(&CONFIG.jwt_secret_key.as_ref()),
        &Validation::default(),
    )
}
