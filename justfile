set shell := ["nu", "-c"]

default: 
    print "test"

alias w := watch
watch:
    cargo watch -c \
        -w src \
        -w templates \
        -w styles \
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

alias c := build 
clean:
    rm static/**/*.css

dock:
    nix build .#dockerImage
    docker load -i result

deploy:
    just dock
    fly deploy --local-only -i aleods-corner
