![math_repl banner](./images/banner.png)

[![crates.io](https://img.shields.io/badge/crates.io-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/math_repl)

math_repl is a REPL/CLI that allows a user to quickly calculate expressions, work with variables as well as various other useful math tools such as integration, an equation solver, etc. Math_repl does not only support real/complex numbers but also vectors and matrices.

The REPL uses the [kitty image protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/) when available to render outputs in a nice latex-like manner. If the protocol is not available in your terminal, the REPL will use a much worse looking and less readable string output.

Without any features enabled you can export the REPL history to a latex document.

When you select the "export" feature when installing you can also export the repl history directly to a pdf or a png. The dependencies used to do this are however very sensitive and might cause build problems on different systems.

When you select the "multithreading" feature when installing the equation solver uses multithreading to greatly speed up equation solving.

:warning: math_repl is built on top of [math_utils_lib](https://crates.io/crates/math_utils_lib), which has not yet reached 1.0.0. Expect breaking changes and bugs.

## Showcase

![A Gif Showcase of the REPL](./images/showcase.gif)

## Installation

You can install math_repl from crates.io.

```
cargo install math_repl
```

If you want to enable the export feature set:

```
cargo install math_repl -F export
```

Make sure that your .cargo/bin is on PATH.

## Usage
Here is some usage information from the --help flag:

```
Simple REPL/CLI for all different kinds of math.

Usage: math_repl [OPTIONS]

Options:
  -e, --eval <EVAL>  Expression(s) to evaluate directly without opening a repl
  -c, --complex      Use complex numbers
  -s, --string-only  Disable kitty image protocol support
  -h, --help         Print help
  -V, --version      Print version
```

To learn what the repl can do, please enter it and type "help".

## Issues and Contributions

If you think you found an issue with the repl itself or want to contribute to it, feel free to open an issue or PR in this repo.

If you think that there is an issue with the math language i.e. the actual parsing/evaluation of prompts, take a look at the [math_utils_lib](https://github.com/Waigo01/math_utils_lib#issues-and-contributions) repo.
