![math_repl banner](./images/banner.png)

[![crates.io](https://img.shields.io/badge/crates.io-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/math_repl)

math_repl is a REPL/CLI that allows a user to quickly calculate expressions, work with variables as well as various other useful math tools such as integration, an equation solver, etc. Math_repl does not only support real/complex numbers but also vectors and matrices. The REPL uses the [kitty image protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/) when available to render outputs in a nice latex-like manner. If the protocol is not available in your terminal, the REPL will use a much worse looking and less readable simple string output.

:warning: math_repl is built on top of [math_utils_lib](https://crates.io/crates/math_utils_lib), which has not yet reached 1.0.0. Expect breaking changes and bugs.

## Showcase

![A Gif Showcase of the REPL](./images/showcase.gif)

## Installation

You can install math_repl from crates.io.

```
cargo install math_repl
```

Make sure that ~/.cargo/bin is on PATH.

## Usage
Here is some usage information from the help command:

```
Simple REPL/CLI for all different kinds of math.

Usage: math_repl [OPTIONS]

Options:
  -e, --eval <EVAL>  Expression(s) to evaluate directly without opening a repl
  -c, --complex      Whether to use complex numbers
  -h, --help         Print help
  -V, --version      Print version
```

To learn what the repl can do, please enter it and type "help".

## Issues and Contributions

If you think you found an issue with the repl itself or want to contribute to it, feel free to open an issue or PR in this repo.

If you think that there is an issue with the math language i.e. the actual parsing/evaluation of prompts, take a look at the [math_utils_lib](https://github.com/Waigo01/math_utils_lib#issues-and-contributions) repo.
