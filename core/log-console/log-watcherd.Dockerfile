FROM rustlang/rust:nightly-slim as builder
WORKDIR /build
COPY . .
RUN apt-get update && apt-get install \
        pkg-config \
        libssl-dev \
        protobuf-compiler \
        libprotobuf-dev \
        -y
RUN cargo build --release --bin log-watcherd

FROM debian:11-slim
RUN apt-get update && apt install libssl1.1 curl -y
COPY --from=builder /build/target/release/log-watcherd /usr/local/bin/log-watcherd
CMD [ "log-watcherd" ]
