# docker build . -t knatnetwork/github-runner-kms-rs:latest-amd64
FROM rust:1.85-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

RUN mkdir src

COPY src ./src

RUN cargo build --release

FROM alpine

WORKDIR /

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/github-runner-kms-rs /github-runner-kms-rs

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000

CMD ["/github-runner-kms-rs"]