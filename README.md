# nv

A language for configuration.

![nv language snippet - neovim](./extensions/vscode-nv/docs/assets/images/vscode-nv-syntax-highlighting.png)

[nv FigJam discovery board](https://www.figma.com/file/YLWiYaLvchfUlrbmr4P0M2/NV-Discovery?type=whiteboard&node-id=0%3A1&t=7aag2YKaHTLDS4lW-1)

## Packages

- Compiler - [Readme](./packages/compiler/README.md)
- Lexer - [Source](./packages/lexer/src/lexer.rs)
- Parser - [Source](./packages/parser/src/main.rs)
- Language Server - **TODO**
- Code Generation - **WIP**
- Code Formatter - **TODO**
- Providers
  - Env [Source]
  - AWS Secrets Manager [Source]
  - AWS KMS - **TODO**
  - GCP Secret Manager - **TODO**
  - Azure Key Vault - **TODO**
  - HasiCorp Vault - **TODO**
- Provider Registry - **TBD**
- Config Registry - **TBD**
- Cloud Service - **TBD**

### Client libraries

- Rust - **WIP**
- TypeScript/JavaScript - **WIP**
- Python - **TODO**
- Go - **WIP**
- C - **TODO**
- C++ - **TODO**
- SWIFT - **TODO**
- CSS - **TBD**

## Extensions

- VSCode - [Grammar](./extensions/vscode-nv/syntaxes/nv.tmLanguage.json)
- treesitter - [Grammar](./extensions/tree-sitter-nv/grammar.js)
- github-linguist - **TODO**

## Examples

- [fullstack-monorepo](./examples/fullstack-monorepo/) - Example use of nv in a monorepo
- [lexer-api](./examples/lexer-api/) - example use of the nv lexer api
- [parser-api](./examples/parser-api/) - example use of the nv parser api
- [nv-files](./examples/nv-files/) - example .nv files of varying complexity
