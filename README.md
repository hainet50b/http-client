# Zed HTTP Client

⚠️ This project is under active development.

A Zed extension for `.http` files, inspired by the HTTP Client in IntelliJ IDEA.

Feature development will follow IntelliJ IDEA's HTTP Client as closely as possible, in both syntax and behavior.

## Features

- Syntax highlighting for `.http` and `.rest` files
- Gutter run button on each request block, which executes the request via a task and shows the response in the integrated terminal

## Acknowledgments

This extension uses the following third-party tree-sitter grammars:

- [`rest-nvim/tree-sitter-http`](https://github.com/rest-nvim/tree-sitter-http) for parsing `.http` files — MIT License, © 2021 NTBBloodbath.
- [`tree-sitter-grammars/tree-sitter-xml`](https://github.com/tree-sitter-grammars/tree-sitter-xml) for highlighting XML request bodies — MIT License, © 2023 ObserverOfTime.
