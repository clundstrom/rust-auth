use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use authio::auth_request::AuthRequest;
use authio::config::CONFIG;
use authio::jwt::{validate_token, JWTClaim};
use authio::traits::authenticate::Authenticate;
use authio::traits::authorize::Authorize;
use authio::{jwt, ldap};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::TokenData;
use log;

/// Endpoint to create a JWT token
///
/// This function is responsible for creating a JWT token. It is an asynchronous function that returns a
/// type that implements the `Responder` trait.
/// This function is mapped to the "/login" route.
///
/// # Steps
///
/// 1. An instance of `LdapAuthenticate` is created with a username and password.
/// 2. The `bind` method of the `LdapAuthenticate` instance is called to authenticate the user against the LDAP server.
/// 3. A JWT token is created for the authenticated user.
/// 4. If the token creation is successful, the token is returned in the response body with an HTTP status of 200.
/// 5. If there is an error during token creation, an error message is logged and an HTTP status of 500 is
/// returned with a generic error message.
#[post("/login")]
async fn create_token(auth: web::Json<AuthRequest>) -> impl Responder {
    // Extract the username and password from the request
    let username = &auth.username;
    let password = &auth.password;

    // Create a new LDAPAuth struct with the username and password from the request
    let mut ldap = ldap::LdapAuthenticate::new();

    // Initialize the LDAP connection
    ldap.initialize().await;
    let create_token = ldap.authenticate(username,password).await;

    // If the user is not authenticated, return an Unauthorized response
    if !create_token {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    // If the user is authenticated, lookup the user's permissions
    let permissions = ldap.resolve_permission(&username).await;

    // Unbind the LDAP connection
    ldap.unbind_ldap().await;

    // Create a JWT token for the user
    let token = match jwt::issue_token(&auth.username, permissions) {
        Ok(token) => token,
        Err(err) => {
            log::error!("Error creating token: {}", err);
            return HttpResponse::InternalServerError()
                .body("Something went wrong. Please try again later.");
        }
    };

    return HttpResponse::Ok().body(token);
}

/// Endpoint to validate a JWT token
///
/// This function is responsible for validating a JWT token. It is an asynchronous function that
/// returns an `HttpResponse`.
/// This function is mapped to the "/validate_request" route.
///
/// # Steps
///
/// 1. The JWT token is extracted from the request's Authorization header using the `extract_token` function.
/// 2. If a token is found, it is validated using the `validate_token` function.
/// 3. The result of the token validation is handled by the `handle_validation_result` function,
/// which returns an appropriate `HttpResponse`.
/// 4. If no token is found, an `HttpResponse::Unauthorized` is returned with a body of "No authorization header found".
///
/// # Arguments
///
/// * `req` - The HttpRequest from which the token is to be extracted and validated.
///
/// # Returns
///
/// * `HttpResponse` - The appropriate HttpResponse based on the token extraction and validation result.
#[get("/validate_request")]
async fn validate_request(req: HttpRequest) -> HttpResponse {
    // Extract the token from the request
    let token = extract_token(req).await;
    match token {
        // If a token is found, validate it
        Some(token_str) => {
            let validation_result = validate_token(token_str).await;
            handle_validation_result(validation_result).await
        }

        // If no token is found, return an Unauthorized response
        None => HttpResponse::Unauthorized().body("No authorization header found"),
    }
}

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

/// Extracts the token from the request's Authorization header.
///
/// # Arguments
///
/// * `req` - The HttpRequest from which the token is to be extracted.
///
/// # Returns
///
/// * `Option<String>` - The extracted token as a String if it exists, None otherwise.
async fn extract_token(req: HttpRequest) -> Option<String> {
    match req.headers().get("Authorization") {
        Some(header_value) => {
            if let Ok(token) = header_value.to_str() {
                Some(token.trim_start_matches("Bearer ").trim().to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

/// Handles the HttpResponse based on the result of the token validation.
///
/// # Arguments
///
/// * `validation_result` - The Result of the token validation.
///
/// # Returns
///
/// * `HttpResponse` - The appropriate HttpResponse based on the token validation result.
async fn handle_validation_result(
    validation_result: Result<TokenData<JWTClaim>, Error>,
) -> HttpResponse {
    match validation_result {
        Ok(_) => HttpResponse::Ok().body("Token valid"),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
            ErrorKind::InvalidKeyFormat => HttpResponse::Unauthorized().body("Invalid key format"),
            ErrorKind::ExpiredSignature => HttpResponse::Unauthorized().body("Token has expired"),
            ErrorKind::InvalidIssuer => HttpResponse::Unauthorized().body("Invalid issuer"),
            ErrorKind::InvalidSubject => HttpResponse::Unauthorized().body("Invalid subject"),
            ErrorKind::InvalidAudience => HttpResponse::Unauthorized().body("Invalid audience"),
            ErrorKind::InvalidSignature => HttpResponse::Unauthorized().body("Invalid signature"),
            ErrorKind::InvalidAlgorithm => HttpResponse::Unauthorized().body("Invalid algorithm"),
            _ => HttpResponse::Unauthorized().body("Invalid request"),
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().service(create_token).service(validate_request))
        .bind((CONFIG.http_bind_address.to_string(), CONFIG.http_port))?
        .run()
        .await
}
