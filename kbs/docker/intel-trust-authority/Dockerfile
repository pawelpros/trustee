FROM rust:latest as builder
ARG HTTPS_CRYPTO=rustls

WORKDIR /usr/src/kbs
COPY . .

RUN apt-get update && apt install -y git

# Build and Install KBS
RUN cargo install --path kbs --bin kbs --no-default-features --features intel-trust-authority-as,${HTTPS_CRYPTO},resource,opa

FROM ubuntu:22.04

LABEL org.opencontainers.image.source="https://github.com/confidential-containers/trustee/kbs"

RUN apt update && apt install -y ca-certificates

COPY --from=builder /usr/local/cargo/bin/kbs /usr/local/bin/kbs
