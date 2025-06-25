# server/Dockerfile
FROM rust:1.78-slim-bookworm AS builder

WORKDIR /usr/src/rust-echo

COPY Cargo.toml Cargo.lock ./

COPY server ./server
COPY client ./client

RUN cargo build --release --workspace --package server

FROM debian:bookworm-slim

COPY --from=builder /usr/src/rust-echo/target/release/server /usr/local/bin/server


EXPOSE 7878

CMD ["/usr/local/bin/server"]