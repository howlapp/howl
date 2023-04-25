FROM ghcr.io/kaylendog/macrostack-services:latest as builder

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=builder /app/target/release/example-service /example-service
ENTRYPOINT ["/example-service"]

LABEL org.opencontainers.image.source=https://github.com/kaylendog/macrostack
LABEL org.opencontainers.image.description="Macrostack example service"
LABEL org.opencontainers.image.licenses=MIT/Apache-2.0
