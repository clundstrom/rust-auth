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
# JWT Settings
JWT_SECRET_KEY=secret
JWT_EXPIRATION_TIME_SECONDS=3600

# LDAP Connection Settings
LDAP_URL=ldap://your-ldap-server:389
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
