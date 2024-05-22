![math_repl banner](./images/banner.png)

[![crates.io](https://img.shields.io/badge/crates.io-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/math_repl)
[![docs.rs](https://img.shields.io/badge/docs.rs-gray?style=for-the-badge&logo=docs.rs)]()

math_repl is a CLI REPL that allows a user to quickly calculate expressions, save the results in variables and use those variables in another expression or equation. It additionally allows a user to solve equations, save its results in variables and use them anywhere. All steps that are taken are recorded and can be exported to LaTeX (see Usage below).

math_repl does not only support numbers but also vectors and matrices.

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
Here is some usage information from the internal help command:

```
You can do 4 basic operations:
        Calculate something: <expr>
        Save the results of a calculation to a variable: <varName> = <expr>
        Solve an equation or a system of equations: eq <expr> = <expr> (, <expr> = <expr>, ...)
        Solve an equation or a system of equations and save it into a variable: <varName> = eq <expr> = <expr> (, <expr> = <expr>, ...)
    As an <expr> counts:
        A scalar (number): <number>
        A vector: [<1>, <2>, ..., <n>]
        A matrix: [[<1:1>, <1:2>, ..., <1:n>], [<2:1>, <2:2>, ..., <2:n>], ..., [<n:1>, <n:2>, ..., <n:n>]] (column major order)
        A Variable: Any previously defined variable.

        You can also use all common operations (see https://docs.rs/math_utils_lib/latest/math_utils_lib/parser/enum.SimpleOpType.html)
        between all different types (It will tell you, when it can't calculate something).
        Additionally there are some advanced operations (see https://docs.rs/math_utils_lib/latest/math_utils_lib/parser/enum.AdvancedOpType.html).
    Additional commands:
        clear: Clears the screen, the history for LaTeX export and all vars except pi and e.
        clearvars: Clears all vars except pi and e.
        vars: Displays all vars.
        export (< --tex | --png >): Exports history since last clear in specified format (leave blank for .pdf).
        help: This help page.
        exit: Exits the REPL.
    Some rules:
        Variable Names must start with an alphabetical letter or a \\. (Greek symbols in LaTeX style get replaced before printing).
        Numbers in Variable Names are only allowed in LaTeX style subscript.
        Any other rules will be explained to you in a (not so) nice manner by the program.
```
