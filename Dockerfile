FROM rust:alpine3.18 as builder
# ENV OPENSSL_DIR="/opt/homebrew/etc/openssl@1.1"
WORKDIR /app
RUN apk add musl-dev
COPY . .
RUN cargo build --release

FROM scratch
USER 1000:1000
COPY --from=builder --chown=1000:1000 /app/target/release/mdpserver /mdpserver
# ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/mdpserver"]