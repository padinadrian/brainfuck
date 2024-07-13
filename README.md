# Brainfuck Interpreter

This is an interpeter for the [brainfuck programming language](https://en.wikipedia.org/wiki/Brainfuck).

## Build

The `cargo` package manager is required to build.

Install rust/cargo: https://www.rust-lang.org/tools/install

Build the project with `cargo`:

```
cargo build
```

## Install

Install with `cargo`:

```
cargo install --path .
```

## Usage

The interpreter requires one argument which is the name of a file containing the input code.

Example:

```
brainfuck <code.bf>
```
