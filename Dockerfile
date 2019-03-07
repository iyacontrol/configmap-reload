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

FROM centos:7.6.1810
RUN wget http://mirrors.ustc.edu.cn/gnu/libc/glibc-2.18.tar.gz && tar -zxvf glibc-2.18.tar.gz && cd glibc-2.18 && mdkir build && cd build && ../configure --prefix=/usr && make -j4 && sudo make install
COPY ./target/release/configmap-reload /configmap-reload
ENTRYPOINT ["/configmap-reload"]