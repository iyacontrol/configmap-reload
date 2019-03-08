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