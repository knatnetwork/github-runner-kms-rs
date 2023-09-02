# docker build . -t knatnetwork/github-runner-kms-rs
FROM rust:1.72 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

RUN mkdir src

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt install -y libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/github-runner-kms-rs /github-runner-kms-rs

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000

CMD ["/github-runner-kms-rs"]