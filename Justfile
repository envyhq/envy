build_environment := "local"

pwd := "$(pwd)/"

clean:
    cargo clean


build:
    cargo build
build-release:
    cargo build --release
