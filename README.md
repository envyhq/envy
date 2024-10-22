# Envy

![CI workflow - test.yml](https://github.com/envyhq/envy/actions/workflows/test.yml/badge.svg?branch=main)

A language for configuration.

|                                                            |                                                                                                             |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| ![Envy logo](./docs/assets/images/discord-server-logo.png) | ![Envy (.nv) syntax highlighting](./extensions/vscode/docs/assets/images/vscode-nv-syntax-highlighting.png) |

[Envy FigJam discovery board](https://www.figma.com/file/YLWiYaLvchfUlrbmr4P0M2/NV-Discovery?type=whiteboard&node-id=0%3A1&t=7aag2YKaHTLDS4lW-1)

Ask a question in GitHub
[discussions](https://github.com/envyhq/envy/discussions),
[issues](https://github.com/envyhq/envy/issues),
[discord](https://discord.gg/tRmdFySx) or email [support@useenvy.cloud](mailto:support@useenvy.cloud)

## Packages

- Lexer - [Source](./packages/lexer/src/lib.rs)
- Parser - [README](./lang/packages/parser/README.md) [Source](./packages/parser/src/lib.rs)
- Resolvers
  - Variable Resolver - [Source](./packages/resolvers/var/src/lib.rs)
  - Provider Resolver - [Source](./packages/resolvers/provider/lib.rs)
- Language Server - [Source](./packages/language-server/src/main.rs)
  - Position Indexer - [Source](./packages/position_indexer/src/lib.rs)
- Code Generation
  - JavaScript - [Source](./packages/code-generation/javascript/src/lib.rs)
- Code Formatter - **TODO**
- Providers
  - Env [Source](./packages/providers/providers/env/src/lib.rs)
  - AWS Secrets Manager [Source](./packages/providers/providers/aws-secrets-manager/src/lib.rs)
  - AWS KMS - **TODO**
  - GCP Secret Manager - **TODO**
  - Azure Key Vault - **TODO**
  - HasiCorp Vault - **TODO**
  - [some_other_provider] - **TODO**
  - [some_distributed_provider] - **TODO**
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

- VSCode
  - [README](./extensions/vscode/README.md)
  - [Grammar](./extensions/vscode/syntaxes/nv.tmLanguage.json)
  - [Language Server Client Source](./extensions/vscode/language-server-client/extension.ts)
- treesitter - [README](./extensions/tree-sitter/README.md) [Grammar](./extensions/tree-sitter/grammar.js)
- github-linguist - **TODO**

## Examples

A lot of packages have an examples/ directory to show
simple usage of the packages API.

## LSP

<https://github.com/envyhq/envy/assets/5678671/e2fd9685-4905-41f7-8cf5-e948d3d7ccb8>](<https://github.com/envyhq/envy/assets/5678671/e2fd9685-4905-41f7-8cf5-e948d3d7ccb8>
