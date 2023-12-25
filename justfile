all:
    earthly --no-sat +all

build:
    earthly --no-sat +build

lint:
    earthly --no-sat +lint

docker:
    earthly --no-sat +docker

push:
    # echo $DOCKERHUB_TOKEN | docker login --username "$DOCKERHUB_USERNAME" --password-stdin
    docker login --username "$DOCKERHUB_USERNAME" --password "$DOCKERHUB_TOKEN"
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
    