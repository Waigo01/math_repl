use math_utils_lib::Context;
use message_handler::handle_message;

pub mod repl;
pub mod message_handler;

pub use crate::repl::Repl;
use crate::repl::State;

pub fn main() {
    let initial_state = State::new(Context::default());
    let mut repl = Repl::new("├ ".to_string(), "│ ".to_string(), initial_state, handle_message);

    let _ = repl.run_repl();
}
