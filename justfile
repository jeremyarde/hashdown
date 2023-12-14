build:
    earthly +build

run: 
    docker run --env-file ./server/.env mdp-server -p 8080:8080

run-fe:
    cd ui-vite && npm run dev