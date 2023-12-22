all:
    earthly --no-sat +all

build:
    earthly --no-sat +build

docker:
    earthly --no-sat +docker

push:
    source .env
    echo $DOCKERHUB_TOKEN | docker login --username "$DOCKERHUB_USERNAME" --password-stdin
    earthly --no-sat --push +docker

test:
    earthly --no-sat +test

rund: 
    docker run --env-file ./server/.env -p 8080:8080 -it mdp-server

runfe:
    cd ui-vite && npm run dev
runbe:
    cd server && cargo r

ls:
    docker run --rm -it testserver ls -l /usr/local/bin

migrate:
    cd server && sqlx migrate revert && sqlx migrate run

runfe:
    cd ui-vite && npm run dev
    