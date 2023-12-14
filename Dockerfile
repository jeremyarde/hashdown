FROM rust:1.74.1-bookworm



COPY ./artifact/target/server /usr/local/bin
COPY ./server/migrations /migrations

EXPOSE 3000
# RUN chmod +x server
ENTRYPOINT ["/usr/local/bin/server"]

# docker build . --tag mdp-server
# docker run --env-file ./server/.env mdp-server