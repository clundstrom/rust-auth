use crate::config::CONFIG;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use jwt::JWTClaim;
use log;
use serde::Deserialize;

mod auth;
mod config;
mod jwt;
mod permission;

/// Endpoint to create a JWT token
#[get("/login")]
async fn create_token() -> impl Responder {
    // Create a new LDAPAuth struct with the username and password from the request
    let mut ldap_auth = auth::LdapAuthenticate::new("username", "password");

    // Bind to the LDAP server with the given credentials
    //let b = ldap_auth.bind().await;

    let token = match jwt::create_token("user_id") {
        Ok(token) => token,
        Err(err) => {
            log::error!("Error creating token: {}", err);
            return HttpResponse::InternalServerError()
                .body("Something went wrong. Please try again later.");
        }
    };

    return HttpResponse::Ok().body(token);
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/create_user")]
async fn create_user(info: web::Json<Info>) -> impl Responder {
    let user = format!("Welcome {}!", info.username);

    HttpResponse::Ok().body(user)
}

/// Test endpoint to validate an authenticated request
#[get("/validate_request")]
async fn validate_request(req: HttpRequest) -> HttpResponse {
    // Extract the token from the request's Authorization header
    match req.headers().get("Authorization") {
        Some(header_value) => {
            if let Ok(token) = header_value.to_str() {
                let token_str = token.trim_start_matches("Bearer ").trim();

                match decode::<JWTClaim>(
                    token_str,
                    &DecodingKey::from_secret(&CONFIG.jwt_secret_key.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => HttpResponse::Ok().body("Token valid").finish(),
                    Err(err) => match *err.kind() {
                        ErrorKind::InvalidToken => {
                            HttpResponse::Unauthorized().body("Invalid token")
                        }
                        ErrorKind::ExpiredSignature => {
                            HttpResponse::Unauthorized().body("Token has expired")
                        }
                        _ => HttpResponse::BadRequest().body("Invalid request"),
                    },
                }
            } else {
                HttpResponse::BadRequest().body("Invalid token format")
            }
        }
        None => HttpResponse::Unauthorized().body("No authorization header found"),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(create_token)
            .service(validate_request)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
