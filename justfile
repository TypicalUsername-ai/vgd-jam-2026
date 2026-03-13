alias b := build

build:
    nix build ./src#default

run: build
    nix run ./src#default
