use math_utils_lib::{Value, Variable};
use message_handler::handle_message;

mod repl;
mod message_handler;

use crate::repl::Repl;

fn main() {
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
