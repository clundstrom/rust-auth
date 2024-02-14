use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

/// Public configuration struct
pub struct Config {
    pub jwt_secret_key: String,
    pub jwt_expiration_time_seconds: u64,
    pub ldap_url: String,
    pub http_bind_address: String,
    pub http_port: u16,
    pub ad_base_dn: String,
    pub ad_filter_format: String,
    pub ad_attrs: Vec<String>,
    pub jwt_company: String,
}

/// Constructor for Config struct that loads the configuration from the environment
///
/// Is made private to ensure that the configuration is only loaded once.
///
/// # Panics
/// Any configuration variables that are not set will cause the program to panic.
/// This is intentional, as the program should not be able to run without the **required** configuration.
impl Config {
    pub fn new() -> Config {
        dotenv().ok();

        let token_expire_seconds_str = env::var("JWT_EXPIRATION_TIME_SECONDS")
            .expect("JWT_EXPIRATION_TIME_SECONDS must be set");
        let token_expiration: u64 = token_expire_seconds_str
            .parse()
            .expect("JWT_EXPIRATION_TIME_SECONDS must be a number");

        Config {
            jwt_secret_key: env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set"),
            jwt_expiration_time_seconds: token_expiration,
            jwt_company: env::var("JWT_COMPANY").expect("JWT_COMPANY must be set"),
            ldap_url: env::var("LDAP_URL").expect("LDAP_URL must be set"),
            http_bind_address: env::var("HTTP_BIND_ADDRESS").expect("HTTP_BIND_ADDRESS must be set"),
            http_port: env::var("HTTP_PORT").expect("HTTP_PORT must be set").parse().unwrap(),
            ad_base_dn: env::var("AD_BASE_DN").expect("AD_BASE_DN must be set"),
            ad_filter_format: env::var("AD_FILTER_FORMAT").expect("AD_FILTER_FORMAT must be set"),
            ad_attrs: env::var("AD_FILTER_ATTRS")
                .expect("AD_FILTER_ATTRS must be set")
                .split(",")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

// Use `lazy_static` to initialize the configuration once and make it globally available.
// This is a good solution for configuration that is read-only and should be available everywhere.
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
