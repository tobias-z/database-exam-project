FROM rust:1.69.0-alpine3.17 as builder
WORKDIR /build
COPY hack/init-conf .
RUN cargo build --release

FROM docker:24-rc-dind
WORKDIR /app
COPY --from=builder /build/target/release/init-conf .
COPY conf conf
COPY hack/start-app .
RUN chmod +x start-app
ENTRYPOINT ["/app/start-app"]
