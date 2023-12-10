# Create the build container to compile the hello world program
FROM messense/rust-musl-cross:aarch64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
# WORKDIR /Users/jarde/Documents/code/markdownparser
WORKDIR /markdownparser

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /markdownparser/recipe.json recipe.json
RUN cargo chef cook --release --target=aarch64-unknown-linux-musl --recipe-path recipe.json 

COPY . .

RUN apt-get update
RUN apt-get install ca-certificates libssl-dev openssl -y

RUN cargo build --release --target=aarch64-unknown-linux-musl

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /markdownparser/target/release/server /server
ENTRYPOINT ["/server"]
EXPOSE 3000