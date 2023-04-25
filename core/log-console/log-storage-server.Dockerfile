FROM rustlang/rust:nightly-slim as builder
WORKDIR /build
COPY . .
RUN apt-get update && apt-get install \
        pkg-config \
        libssl-dev \
        protobuf-compiler \
        libprotobuf-dev \
        -y
RUN cargo build --release --bin log-storage-server

FROM debian:11-slim
RUN apt-get update && apt install libssl1.1 -y
COPY --from=builder /build/target/release/log-storage-server /usr/local/bin/log-storage-server
# If we don't do this we will not be able to connect to the server from other places than inside the container
ENV ROCKET_ADDRESS=0.0.0.0
CMD [ "log-storage-server" ]
