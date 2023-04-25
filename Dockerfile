FROM rust:slim-buster

RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev protobuf-compiler

WORKDIR /app

COPY . .

RUN cargo build -r --workspace --bins --exclude howl-prisma-cli

LABEL org.opencontainers.image.source=https://github.com/howlapp/howl
LABEL org.opencontainers.image.description="Howl services image"
LABEL org.opencontainers.image.licenses=MIT/Apache-2.0
