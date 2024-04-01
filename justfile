set shell := ["nu", "-c"]

default: 
    @just --list

watch:
    cargo watch -c \
        -w src \
        -w templates \
        -w styles \
        -w static \
        -s "just run" 


styles:
    sass --no-source-map styles/globals.scss:static/globals.css styles/pages:static/pages

build: styles
    cargo build
    
run: build
    cargo run

kill:
    ps | where name =~ server | each {kill $in.pid}


# Kill docker container
dk:
    # Kill containers.
    docker ps --format json  \
        | lines | each { from json } \
        | where Image == aleods-corner:latest \
        | get ID \
        | each { \
            docker kill $in \
        };
# Remove docker image
drm:
    # Remove image.
    docker images --format json \
        | lines | each { from json } \
        | where Repository == aleods-corner \
        | where Tag == latest \
        | get ID \
        | each { \
            docker rmi $in \
        };

# Load docker image
dl:
    nix build .#dockerImage
    docker load -i result

# kill rm and load docker image
db:
    @just dk
    @just drm
    @just dl

# Build new image and run it
dr:
    @just db
    docker run -d --rm --publish 8080:8080 aleods-corner:latest 

# Watch and reload docker image 
dw: 
    cargo watch -c \
        -w src \
        -w templates \
        -w styles \
        -w static \
        -s "just dr" 

# Exec cmd in running docker image 
dexec cmd: 
    docker ps --format json  \
        | lines | each { from json } \
        | where Image == aleods-corner:latest \
        | get ID \
        | each { \
            docker exec $in {{cmd}}\
        };

# Deploy current docker image  to fly.io
deploy:
    just db
    fly deploy --local-only -i aleods-corner
