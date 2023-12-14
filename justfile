build:
    earthly +build

docker:
    earthly +docker

run: 
    docker run --env-file ./server/.env -p 8080:8080 mdp-server 

run-fe:
    cd ui-vite && npm run dev