FROM arm64v8/rust:1.69.0-alpine3.17 as builder
WORKDIR /build
COPY hack/init-conf .
RUN apk update && apk add --no-cache musl-dev
RUN cargo build --release

FROM arm64v8/docker:24-rc-dind
WORKDIR /app
RUN apk update && apk add nginx nginx-mod-stream
COPY hack/nginx-conf/nginx.conf /etc/nginx/nginx.conf
COPY hack/nginx-conf/http.d /etc/nginx/http.d
COPY hack/nginx-conf/stream.d /etc/nginx/stream.d
COPY --from=builder /build/target/release/init-conf .
COPY hack/wait-for-container.sh /usr/local/bin/wait-for-container.sh
RUN chmod +x /usr/local/bin/wait-for-container.sh
COPY conf conf
COPY hack/start-app .
RUN chmod +x start-app
ENTRYPOINT ["/app/start-app"]
