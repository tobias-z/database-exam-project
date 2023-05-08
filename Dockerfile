FROM rust:1.69.0-alpine3.17 as builder
WORKDIR /build
COPY hack/init-conf .
RUN cargo build --release

FROM docker:24-rc-dind
WORKDIR /app
RUN apk update && apk add nginx nginx-mod-stream
COPY hack/nginx-conf/nginx.conf /etc/nginx/nginx.conf
COPY hack/nginx-conf/http.d /etc/nginx/http.d
COPY hack/nginx-conf/stream.d /etc/nginx/stream.d
COPY --from=builder /build/target/release/init-conf .
COPY conf conf
COPY hack/start-app .
RUN chmod +x start-app
ENTRYPOINT ["/app/start-app"]
