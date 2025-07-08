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

fe:
    cd ui && npm run dev
be:
    cd server && cargo r

ls:
    docker run --rm -it testserver ls -l /usr/local/bin

migrateprodf:
    # cd server && source .prod.env && sqlx migrate revert --database-url $DATABASE_URL && sqlx migrate run --database-url $DATABASE_URL

gentypes:
    supabase gen types typescript --project-id vbvounbggaxtaofatdyg > ui/src/types/supabase.ts

buildwasm:
    cd backend && wasm-pack build --target bundler
    
buildfe:
    cd ui && npm run build

seamigrate:
    sea-orm-cli migrate refresh

seamigrateprod:
    source server/.prod.env && sea-orm-cli migrate refresh --database-url $DATABASE_URL

sea:
    sea-orm-cli migrate refresh && cd server && sea-orm-cli generate entity --with-serde both -s mdp -o ../entity/src/entities

