build:
    earthly +build

run: 
    docker run --env-file ./server/.env mdp-server

run-fe:
    cd ui-vite && npm run dev