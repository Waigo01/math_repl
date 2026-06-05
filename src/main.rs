use math_utils_lib::Context;
use message_handler::handle_message;

pub mod repl;
pub mod message_handler;
mod cli;

pub use crate::repl::Repl;
use crate::{cli::handle_expressions, repl::State};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Expression(s) to evaluate
    #[arg(short, long, required=false)]
    eval: Option<Vec<String>>
}

pub fn main() {
    let args = Args::parse();

    match args.eval {
        None => {
            let initial_state = State::new(Context::default());
            let mut repl = Repl::new("-> ".to_string(), "   ".to_string(), initial_state, handle_message);

            let _ = repl.run_repl();
        },
        Some(expressions) if expressions.len() == 0 => {
            let initial_state = State::new(Context::default());
            let mut repl = Repl::new("-> ".to_string(), "   ".to_string(), initial_state, handle_message);

            let _ = repl.run_repl();
        },
        Some(expressions) => {
            handle_expressions(expressions);
        }
    }
}
