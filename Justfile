build_environment := "local"

pwd := "$(pwd)/"

clean:
    cargo clean
    rm -rf **/node_modules node_modules

install:
    pnpm install
    pnpm dev:install

build:
    pnpm build
    cargo build
build-release:
    cargo build --release
