# Build Rust application
FROM rust:1.75

WORKDIR /usr/src/authio
COPY . .

RUN cargo install --path .

CMD ["authio"]
