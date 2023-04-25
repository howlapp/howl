FROM ghcr.io/howlapp/services:latest as builder

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/target/release/gateway /gateway
ENTRYPOINT ["/gateway"]

LABEL org.opencontainers.image.source=https://github.com/howlapp/howl
LABEL org.opencontainers.image.description="Howl backend gateway"
LABEL org.opencontainers.image.licenses=MIT/Apache-2.0
