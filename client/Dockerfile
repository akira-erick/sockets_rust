# client/Dockerfile
FROM rust:1.78-slim-bookworm AS builder

WORKDIR /usr/src/rust-echo

COPY Cargo.toml Cargo.lock ./

COPY server ./server
COPY client ./client

RUN cargo build --release --workspace --package client

FROM debian:bookworm-slim

COPY --from=builder /usr/src/rust-echo/target/release/client /usr/local/bin/client


ENTRYPOINT ["/usr/local/bin/client"]