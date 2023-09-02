# docker build . -t knatnetwork/github-runner-kms-rs:latest-amd64
FROM rust:1.72 as builder

WORKDIR /app

RUN apt update && apt install musl-tools pkg-config libssl-dev -y && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

RUN mkdir src

COPY src ./src

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/github-runner-kms-rs /github-runner-kms-rs

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000

CMD ["/github-runner-kms-rs"]