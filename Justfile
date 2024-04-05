build_environment := "local"

pwd := "$(pwd)/"

clean:
    cargo clean
    rm -rf **/node_modules node_modules

setup:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo install --locked pueue
    just install
    just build

install:
    pnpm install
    pnpm dev:install

pueue-init:
    pkill pueued
    pueued &
    pueue clean
pueue-result:
    pueue status
    pueue wait
    pueue status

build:
    pnpm i
    just pueue-init
    pueue add pnpm build
    pueue add cargo build
    just pueue-result
build-release:
    cargo build --release
