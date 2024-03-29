#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("banner", "images/banner.png"),
doc = ::embed_doc_image::embed_image!("showcase", "images/showcase.gif")))]
#![cfg_attr(
not(feature = "doc-images"),
doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//! ![math_repl banner][banner]
//!
//! math_repl is a CLI REPL that allows a user to quickly calculate expressions, save the results in variables and use those variables in another expression or equation. It additionally allows a user to solve equations, save its results in variables and use them anywhere. All steps that are taken are recorded and can be exported to LaTeX (see Usage below).
//!
//! math_repl does not only support numbers but also vectors and matrices.
//!
//! <div class = "warning"> math_repl is built on top of <a href="https://crates.io/crates/math_utils_lib">math_utils_lib</a>, which has not yet reached 1.0.0. Expect breaking changes and bugs. </div>
//!
//! ## Showcase
//!
//! ![A Gif Showcase of the REPL][showcase]
//!
//! ## Installation
//!
//! You can install math_repl from crates.io.
//!
//! ```
//! cargo install math_repl
//! ```
//!
//! Make sure that ~/.cargo/bin is on PATH.
//!
//! ## Usage
//! 
//! Here is some usage information from the internal help command:
//!
//! ```text
//! Usage:
//! You can do 4 basic operations:
//!     Calculate something: <expr>
//!     Save the results of a calculation to a variable: <varName> = <expr>
//!     Solve an equation (using x as variable to solve for): eq <expr> = <expr>
//!     Solve an equation and save it into a variable (using <varName> as variable to solve for): <varName> = eq <expr> = <expr>
//! As an <expr> counts:
//!     A scalar (number): <number>
//!     A vector: [<1>, <2>, ..., <n>]
//!     A matrix: [[<1:1>, <1:2>, ..., <1:n>], [<2:1>, <2:2>, ..., <2:n>], ..., [<n:1>, <n:2>, ..., <n:n>]]
//!     A Variable: Any previously defined variable.
//!
//!     You can also use all common operations (see https://docs.rs/math_utils_lib/latest/math_utils_lib/parser/enum.OpType.html)
//!     between all different types (It will tell you, when it can't calculate something).
//! Additional commands:
//!     clear: Clears the screen, the history for LaTeX export and all vars except pi and e.
//!     clearvars: Clears all vars except pi and e.
//!     vars: Displays all vars.
//!     export (< --tex | --png >): Exports history since last clear in specified format (leave blank for .pdf).
//!     help: This help page.
//!     exit: Exits the REPL.
//! Some rules:
//!     Variable Names must start with an alphabetical letter or a \\. (Greek symbols in LaTeX style get replaced before printing).
//!     Numbers in Variable Names are only allowed in LaTeX style subscript.
//!     Any other rules will be explained to you in a (not so) nice manner by the program.
//! ```

use math_utils_lib::{Value, Variable};
use message_handler::handle_message;

pub mod repl;
pub mod message_handler;

pub use crate::repl::Repl;

pub fn main() {
    let initial_state = (vec![ 
        Variable {
            name: "pi".to_string(),
            value: Value::Scalar(std::f64::consts::PI)
        },
        Variable {
            name: "e".to_string(),
            value: Value::Scalar(std::f64::consts::E)
        }
    ], vec![]);
    let mut repl = Repl::new("├ ".to_string(), "│ ".to_string(), initial_state, handle_message);

    let _ = repl.run_repl();
}
