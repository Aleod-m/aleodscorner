set shell := ["nu", "-c"]

default: 
    @just --list

alias w := watch
watch:
    cargo watch -c \
        -w src \
        -w templates \
        -w styles \
        -w static \
        -s "just run" 


alias s := styles 
styles:
    sass --no-source-map styles/globals.scss:static/globals.css styles/pages:static/pages

alias b := build 
build: styles
    cargo build
    
alias r := build 
run: build
    cargo run

alias k := kill
kill:
    ps | where name =~ server | each {kill $in.pid}

db:
    nix build .#dockerImage
    # Kill containers.
    docker ps --format json  \
        | lines | each { from json } \
        | where Image == aleods-corner:latest \
        | get ID \
        | each { \
            docker kill $in \
        };
    # Remove image.
    docker images --format json \
        | lines | each { from json } \
        | where Repository == aleods-corner \
        | where Tag == latest \
        | get ID \
        | each { \
            docker rmi $in \
        };

    docker load -i result

dr:
    @just db
    docker run -d --rm --publish 8080:8080 aleods-corner:latest 

dw: 
    cargo watch -c \
        -w src \
        -w templates \
        -w styles \
        -w static \
        -s "just dr" 

dexec cmd: 
    docker ps --format json  \
        | lines | each { from json } \
        | where Image == aleods-corner:latest \
        | get ID \
        | each { \
            docker exec $in {{cmd}}\
        };

deploy:
    just db
    fly deploy --local-only -i aleods-corner
