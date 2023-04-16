FROM rust:slim-buster as builder

RUN USER=root cargo new --bin rust-docker
WORKDIR ./rust-docker
COPY ./ ./

RUN apt-get update \
    && apt-get install -y libpq-dev \
    && rustup override set nightly \
    && cargo build --release

FROM debian:buster-slim

WORKDIR /app

ENV TZ=Etc/UTC
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8910

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /app

COPY --from=builder /rust-docker/target/release/wtf-rocket /app

CMD ["./wtf-rocket"]