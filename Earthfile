VERSION --global-cache 0.7
# VERSION 0.7
ARG run_locally=true


IMPORT github.com/earthly/lib/rust AS rust

install:
  FROM rust:1.74.1-bookworm
  RUN rustup component add clippy rustfmt

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
  FROM +lint
  DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
  SAVE ARTIFACT ./target/release/ target AS LOCAL artifact/target

docker:
  # FROM rust:1.74.1-bookworm # works
  # FROM debian:12 # works with libssl-dev
  # RUN apt-get update && apt-get install -y libssl-dev
  # IF [ "$run_locally" = "true" ]
  #   LOCALLY
  # ELSE
  FROM debian:bookworm-slim # does work with libssl-dev
  RUN apt-get update && apt-get install -y libssl-dev

  # FROM scratch
  # FROM gcr.io/distroless/cc
  # RUN apt-get update && apt-get install -y libssl-dev
  WORKDIR /myapp
  COPY +build/target/mdpserver /myapp
  EXPOSE 8080
  CMD ["./mdpserver"]
  SAVE IMAGE mdp-server:latest

# test executes all unit and integration tests via Cargo
test:
  LOCALLY
  FROM +lint
  DO rust+CARGO --args="test"

# fmt checks whether Rust code is formatted according to style guidelines
fmt:
  LOCALLY
  FROM +lint
  DO rust+CARGO --args="fmt --check"

# all runs all other targets in parallel
all:
  LOCALLY
  BUILD +fmt
  BUILD +build
  BUILD +docker
  # BUILD +test
  # BUILD +docker