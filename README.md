# nv

A language for application/program/system configs.

![nv language snippet - neovim](./extensions/vscode-nv/docs/assets/images/vscode-nv-syntax-highlighting.png)

## Packages

- Compiler - [Readme](./packages/compiler/README.md)
- Lexer - [Source](./packages/lexer/src/lexer.rs)
- Parser - [Source](./packages/parser/src/main.rs)
- Language Server - **TODO**

### Client libraries

- Rust - **WIP**
- TypeScript/JavaScript - **WIP**
- Python - **TODO**
- Go - **TODO**

## Extensions

- VSCode - [Grammar](./extensions/vscode-nv/syntaxes/nv.tmLanguage.json)
- treesitter - [Grammar](./extensions/tree-sitter-nv/grammar.js)
- github-linguist - **TODO**

## Examples

- [fullstack-monorepo](./examples/fullstack-monorepo/) - Example use of nv in a monorepo
- [lexer-api](./examples/lexer-api/) - Example use of the nv lexer API
