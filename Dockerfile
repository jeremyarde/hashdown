FROM rust:1.74.1-bookworm



COPY ./artifact/target/server server
COPY ./server/migrations /migrations

EXPOSE 3000
# RUN chmod +x server
ENTRYPOINT ["./server"]

# docker build . --tag mdp-server
# docker run --env-file ./server/.env mdp-server