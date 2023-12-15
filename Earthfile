VERSION --global-cache 0.7

IMPORT github.com/earthly/lib/rust AS rust

# FROM rust:slim-buster
# WORKDIR /

install:
  FROM rust:1.74.1-slim-bookworm
  RUN rustup target add x86_64-unknown-linux-musl
  RUN apt update && apt install -y musl-tools musl-dev
  RUN apt-get install -y build-essential
  RUN rustup component add clippy rustfmt

  ENV HOST_CC='gcc'
  ENV CC_x86_64_unknown_linux_gnu='/usr/bin/x86_64-linux-gnu-gcc'
  ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
  # for Cargo
  # ENV CARGO_TARGET_X86_64-UNKNOWN-LINUX-GNU_LINKER /usr/bin/x86_64-linux-gnu-gcc

  # ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
  # RUN rustup target add x86_64-unknown-linux-musl
  # RUN apt update && apt install -y musl-tools musl-dev
  # Call +INIT before copying the source file to avoid installing function depencies every time source code changes
  # This parametrization will be used in future calls to functions of the library
  DO rust+INIT --keep_fingerprints=true

source:
  FROM +install
  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --dir server backend ./

# lint runs cargo clippy on the source code
lint:
  FROM +source
  # DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"
  DO rust+CARGO --args="clippy --all-features --all-targets"

build:
    FROM rust:1.74.1-slim-bookworm
    RUN rustup target add x86_64-unknown-linux-musl
    RUN apt update && apt install -y musl-tools musl-dev
    # Cargo UDC adds caching to cargo runs.
    # See https://github.com/earthly/lib/tree/main/rust
    FROM +source
    DO rust+CARGO --args="build --target x86_64-unknown-linux-musl --release" --output="release/[^/\.]+"
    SAVE ARTIFACT target/release/server server

# docker creates docker image earthly/examples:rust
docker:
  FROM alpine
    # FROM rust:1.74.1-slim-bookworm
    COPY +build/server server
    EXPOSE 8080
    ENTRYPOINT ["./server"]
    SAVE IMAGE mdp-server:latest

# test executes all unit and integration tests via Cargo
test:
  FROM +lint
  DO rust+CARGO --args="test"

# fmt checks whether Rust code is formatted according to style guidelines
fmt:
  FROM +lint
  DO rust+CARGO --args="fmt --check"

# all runs all other targets in parallel
all:
  BUILD +build
  BUILD +test
  BUILD +fmt