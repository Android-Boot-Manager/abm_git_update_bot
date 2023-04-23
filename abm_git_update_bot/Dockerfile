FROM rust:latest as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/bootstrap /app/bootstrap

ENTRYPOINT ["/app/bootstrap"]
