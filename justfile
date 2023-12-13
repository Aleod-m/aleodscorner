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

alias c := build 
clean:
    rm static/**/*.css

dock:
    nix build .#dockerImage
    docker load < ./result
