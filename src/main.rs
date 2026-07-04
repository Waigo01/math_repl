use math_utils_lib::{Complex, Context};
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
    /// Expression(s) to evaluate directly without opening a repl
    #[arg(short, long, required=false)]
    eval: Option<Vec<String>>,
    /// Use complex numbers
    #[arg(short, long, required=false)]
    complex: bool,
    /// Disable kitty image protocol support
    #[arg(short, long, required=false)]
    string_only: bool
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();

    if let Some(expressions) = args.eval && expressions.len() != 0 {
        if args.complex {
            handle_expressions::<Complex<f64>>(expressions);
        } else {
            handle_expressions::<f64>(expressions);
        }
    } else {
        if args.complex {
            let initial_state: State<Complex<f64>> = State::new(Context::default());
            let mut repl = Repl::new("→ ".to_string(), "  ".to_string(), initial_state, handle_message, !args.string_only);

            let _ = repl.run_repl().await;
        } else {
            let initial_state: State<f64> = State::new(Context::default());
            let mut repl = Repl::new("→ ".to_string(), "  ".to_string(), initial_state, handle_message, !args.string_only);

            let _ = repl.run_repl().await;
        }
    }
}
