# FROM rust:1.33.0 as builder
# WORKDIR /app/configmap-reload
# COPY ./ ./
# RUN cargo build --release

# FROM debian:stable-slim
# RUN apt update \
#     && apt install -y openssl ca-certificates \
#     && apt clean \
#     && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
# COPY --from=builder /app/configmap-reload/target/release/configmap-reload /usr/bin/configmap-reload
# RUN chmod +x /usr/bin/configmap-reload

# FROM gcr.io/distroless/cc
# COPY --from=builder /app/configmap-reload/target/release/configmap-reload /configmap-reload
# ENTRYPOINT ["/configmap-reload"]

# FROM gcr.io/distroless/cc
# COPY ./target/release/configmap-reload /configmap-reload
# ENTRYPOINT ["/configmap-reload"]

# FROM alpine:3.9
# COPY ./target/release/configmap-reload /configmap-reload
# ENTRYPOINT ["/configmap-reload"]

# FROM rustlang/rust:nightly-slim  as builder
# WORKDIR /app/configmap-reload
# COPY ./ ./
# RUN cargo build --release

# FROM debian:stable-slim
# RUN apt update \
#     && apt install -y openssl ca-certificates \
#     && apt clean \
#     && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
# COPY --from=builder /app/configmap-reload/target/release/configmap-reload /configmap-reload
# ENTRYPOINT ["/configmap-reload"]


FROM frolvlad/alpine-rust
WORKDIR /app/configmap-reload
COPY ./ ./
RUN cargo build --release
ENTRYPOINT ["/app/configmap-reload/target/release/configmap-reload"]