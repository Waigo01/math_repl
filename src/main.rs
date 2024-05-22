use math_utils_lib::{Value, Variable};
use message_handler::handle_message;

pub mod repl;
pub mod message_handler;

pub use crate::repl::Repl;

pub fn main() {
    let initial_state = (vec![ 
        Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI)),
        Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E)),
    ], vec![]);
    let mut repl = Repl::new("├ ".to_string(), "│ ".to_string(), initial_state, handle_message);

    let _ = repl.run_repl();
}
