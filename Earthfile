VERSION --global-cache 0.7

IMPORT github.com/earthly/lib/rust AS rust

# FROM rust:slim-buster
# WORKDIR /

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

# build builds with the Cargo release profile
build:
  FROM +lint
  DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
  SAVE ARTIFACT ./target/release/ target AS LOCAL artifact/target

docker:
  FROM rust:1.74.1-bookworm
  # FROM +build
  COPY +build/target /usr/local/bin 
  # RUN cargo install --path .
  EXPOSE 3000
  CMD ["server"]
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