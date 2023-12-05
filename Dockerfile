# Create the build container to compile the hello world program
FROM rust:1.74.0 as builder
# ENV USER root
# RUN cargo new hello
WORKDIR /Users/jarde/Documents/code/markdownparser
COPY ./ ./
RUN cargo build --release

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /markdownparser/target/release/server /server
CMD ["/hello"]