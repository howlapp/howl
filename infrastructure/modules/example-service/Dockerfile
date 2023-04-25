FROM ghcr.io/howlapp/services:latest as builder

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/target/release/example-service /example-service
ENTRYPOINT ["/example-service"]

LABEL org.opencontainers.image.source=https://github.com/kaylendog/howlapp
LABEL org.opencontainers.image.description="Howl example service"
LABEL org.opencontainers.image.licenses=MIT/Apache-2.0
