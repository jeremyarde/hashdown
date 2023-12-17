build:
    earthly --no-sat +build

docker:
    earthly --no-sat +docker

test:
    earthly --no-sat +test

rund: 
    docker run --env-file ./server/.env -p 8080:8080 -it mdp-server

runl:
    cd server && cargo r

run-fe:
    cd ui-vite && npm run dev

other:
    docker run --rm -it mdp-server mdpserver

ls:
    docker run --rm -it testserver ls -l /usr/local/bin
