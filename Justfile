build_environment := "local"

pwd := "$(pwd)/"

clean:
    cargo clean
    rm -rf **/node_modules **/**/node_modules node_modules
    rm -rf **/dist

setup:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo install --locked pueue
    just install
    just build

dev-install:
    just pueue-init
    pueue add "pnpm -r dev:install"
    pueue add "cd extensions/vscode-nv && npm run dev:install"
    just pueue-result
install:
    just pueue-init
    pueue add "pnpm install"
    pueue add "cd extensions/vscode-nv && npm i"
    just pueue-result
    just dev-install

pueue-init:
    pkill pueued || true
    pueued &
    pueue clean
pueue-result:
    pueue status
    pueue wait
    pueue status
    pueue log

build:
    pnpm i
    just pueue-init
    pueue add "pnpm -r build"
    pueue add "cd extensions/vscode-nv && npm run build"
    pueue add "cargo build"
    just pueue-result
build-release:
    cargo build --release

lsp:
  cd packages/language-server
  cargo watch -x "run --bin nv-language-server"
