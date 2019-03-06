FROM rust:1.33.0 as builder
WORKDIR /app/configmap-reload
COPY ./ ./
RUN cargo build --release

FROM alpine:3.9
COPY --from=builder /app/configmap-reload/target/release/configmap-reload /usr/bin/configmap-reload
RUN chmod +x /usr/bin/configmap-reload



