# Rust authentication service

## Description

This is a simple authentication service written in Rust. It uses the [actix-web](https://actix.rs/) framework.

## Usage

### Running the service

To run the service, you need to have Rust installed. You can install Rust by following the
instructions [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can run the service by executing the following command:

### Environment variables

The service uses the following environment variables:

#### Required
```bash
JWT_SECRET_KEY=your_secret_key
JWT_EXPIRATION_TIME_SECONDS=3600
JWT_COMPANY="Example AB"
LDAP_URL=ldap://localhost:389

# HTTP Server
HTTP_BIND_ADDRESS=0.0.0.0
HTTP_PORT=8080

# Base directory for the LDAP search
AD_BASE_DN=ou=people,dc=example,dc=com
AD_FILTER_FORMAT=(&(objectClass=*))
AD_FILTER_ATTRS=*,memberOf
```

#### Optional
```bash
# Log Level Settings
# Possible values: trace, debug, info, warn, error (default: info)
# Can be set to a specific crate, e.g. RUST_LOG=debug,my_crate=info
RUST_LOG=debug
```

```bash
cargo run
```

### Running the tests

To run the tests, you can execute the following command:

```bash
cargo test
```
### Building the documentation

To build the documentation, you can execute the following command:

```bash
cargo doc
```

### Running the benchmarks

To run the benchmarks, you can execute the following command:

```bash
cargo bench
```

### Testing the service

You can test the service by sending a POST request to the `/login` endpoint with a JSON body containing the `username` and `password` fields.

Example:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"username": "user", "password": "password"}' http://localhost:8080/login
```

The service will respond with a JWT token if the credentials are correct.

This token can then be used to access the `/validate_request` endpoint.

Example:

```
curl -X GET -H "Authorization: Bearer <token>" http://localhost:8080/validate_request
```

On successful validation, the service will respond with a `200 OK` status code and the body Token valid.
