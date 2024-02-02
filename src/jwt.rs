use crate::config::CONFIG;
use crate::permission::Permission;
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
pub(crate) struct JWTClaim {
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
/// let token = create_token("user123");
/// match token {
///     Ok(t) => println!("Token: {}", t),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub(crate) fn issue_token(user_id: &str) -> Result<String, Error> {
    // The expiration time for the token is retrieved from the configuration.
    let expiration_seconds: &u64 = &CONFIG.jwt_expiration_time_seconds;
    // The expiration time is calculated by adding the expiration seconds to the current time.
    let expiration_time = Utc::now() + Duration::seconds(*expiration_seconds as i64);

    // The claims for the JWT token are created. The subject is the user id, the company is hardcoded
    // (this should be moved to an environment variable), the expiration time is set to the calculated expiration time,
    // and the permissions are currently an empty vector (this should be set to the user's permissions).
    let claims = JWTClaim {
        sub: user_id.to_owned(),
        company: "Dippen preb AB".to_owned(), // TODO: move to envvar
        exp: expiration_time.timestamp() as usize,
        permissions: vec![], // TODO: set permissions
    };


    log::debug!("Issue: {:?}", claims);
    // The secret key for the JWT token is retrieved from the configuration.
    let secret = &CONFIG.jwt_secret_key;

    // The secret key is used to create an encoding key for the JWT token.
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    // The JWT token is encoded with the default header, the created claims, and the encoding key.
    let issued_token = encode(&Header::default(), &claims, &encoding_key);
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
pub(crate) async fn validate_token(
    token_str: String,
) -> jsonwebtoken::errors::Result<TokenData<JWTClaim>> {
    decode::<JWTClaim>(
        &token_str,
        &DecodingKey::from_secret(&CONFIG.jwt_secret_key.as_ref()),
        &Validation::default(),
    )
}
