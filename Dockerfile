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


# FROM frolvlad/alpine-rust
# RUN apk add --update openssl && \
#     rm -rf /var/cache/apk/*
# WORKDIR /app/configmap-reload
# COPY ./ ./
# RUN cargo build --release
# ENTRYPOINT ["/app/configmap-reload/target/release/configmap-reload"]

FROM clux/muslrust:stable as builder

WORKDIR /configmap-reload
COPY ./ ./

ARG use_mirror
RUN if [ $use_mirror ]; then \
        mkdir -p $HOME/.cargo; \
        mv -f ./docker/cargo_config  $HOME/.cargo/config; \
    fi
RUN cargo build --release

#####################################

FROM alpine:latest as prod

RUN apk add --no-cache ca-certificates 

COPY --from=0 /configmap-reload/target/x86_64-unknown-linux-musl/release/configmap-reload /usr/bin/configmap-reload
RUN chmod +x /usr/bin/configmap-reload

ENTRYPOINT ["configmap-reload"]