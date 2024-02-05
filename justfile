set shell := ["nu", "-c"]

default: 
    print "test"

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
    docker load -i result
dr:
    @just db
    docker run -d --rm --publish 8080:8080 aleods-corner:latest 

deploy:
    just db
    fly deploy --local-only -i aleods-corner
