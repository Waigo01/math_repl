use std::{any::Any, error::Error, io::Write};

use console::{style, Key, Term};
use math_utils_lib::MathLibError;

/// describes internal commands for [Repl] to execute.
pub enum Exec {
    Exit,
    Clear
}

/// describes the type of action that should be taken by [Repl].
pub enum Action {
    Print(String),
    Exec(Exec)
}

/// describes a simple HandlerError type.
pub struct HandlerError {
    pub message: String
}

impl From<MathLibError> for HandlerError {
    fn from(value: MathLibError) -> Self {
        HandlerError { message: value.get_reason() }
    }
}

/// describes the REPL
///
/// A REPL can be initialized using the [new()](fn@Repl::new()) method. This method requires an input_prefix, which
/// is printed before any input is taken, an output_prefix, which is printed before any output is printed,
/// an initial_state, which is described by the Generic T and a handler, which is a function of type
/// ```FnMut(String, &mut T) -> Result<Action, HandlerError>```.
///
/// After the initialization a REPL can be run using the [run_repl()](fn@Repl::run_repl()) method.
///
/// # Example
///
/// ```
/// let initial_state: Vec<String> = vec![];
///
/// fn message_handler(input: String, global_state: &mut Vec<String>) -> Result<Action, HandlerError> {
///     if global_state.len() > 3 {
///         return Ok(Action::Exec(Exec::Exit));
///     } else {
///         global_state.push("Hello World".to_string());
///         return Ok(Action::Print(format!("{}: {}", global_state[global_state.len()-1], input)));
///     }
/// }
///
/// let mut repl = Repl::new("> ".to_string(), "| ".to_string(), initial_state, message_handler);
///
/// repl.run_repl()?;
///```
///
///This crude message_handler will take four inputs and print back "Hello World: \<input\>". It
///will exit on the fifth input.
pub struct Repl<T: Any + Clone, F: FnMut(String, &mut T) -> Result<Action, HandlerError>> {
    term: Term,
    input_prefix: String,
    output_prefix: String,
    pub message_handler: F,
    pub global_state: T
}

impl<T: Any + Clone, F: FnMut(String, &mut T) -> Result<Action, HandlerError>> Repl<T, F> {
    /// used to initialize a new [Repl].
    pub fn new(input_prefix: String, output_prefix: String, initial_state: T, handler: F) -> Repl<T, F> {
        Repl {
            term: Term::stdout(),
            input_prefix,
            output_prefix,
            message_handler: handler,
            global_state: initial_state
        }
    }
    /// used to run a [Repl].
    pub fn run_repl(&mut self) -> Result<(), Box<dyn Error>> {
        let mut history: Vec<String> = vec![];
        self.term.clear_screen()?;
        loop {
            self.term.write(self.input_prefix.as_bytes())?;
            self.term.flush()?;
            let mut input_buffer = String::new();
            let mut position = 0;
            let mut history_pos = -1;
            loop {
                match self.term.read_key()? {
                    Key::Char(c) => {
                        if position == input_buffer.len() {
                            input_buffer.push(c);
                        } else {
                            input_buffer.insert(position as usize, c);
                        }
                        position += 1;
                        self.term.move_cursor_right(input_buffer.len()-position)?;
                        self.term.clear_chars(input_buffer.len()-1)?;
                        self.term.write(input_buffer.as_bytes())?;
                        self.term.move_cursor_left(input_buffer.len()-position)?;
                    },
                    Key::ArrowLeft => {
                        if position as i32-1 >= 0 {
                            self.term.move_cursor_left(1)?;
                            position -= 1;
                        } 
                    },
                    Key::ArrowRight => {
                        if position+1 <= input_buffer.len() {
                            self.term.move_cursor_right(1)?;
                            position += 1;
                        }
                    },
                    Key::Backspace => {
                        if position as i32-1 >= 0 {
                            input_buffer.remove(position-1);
                            position -= 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len()+1)?;
                            self.term.write(input_buffer.as_bytes())?;
                            self.term.move_cursor_left(input_buffer.len()-position)?;
                        } 
                    },
                    Key::ArrowUp => {
                        if history_pos + 1 < history.len() as i32 && history_pos + 1 >= 0 {
                            history_pos += 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len())?;
                            input_buffer = history[history_pos as usize].clone(); 
                            self.term.write(input_buffer.as_bytes())?;
                            position = input_buffer.len();
                        }
                    },
                    Key::ArrowDown => {
                        if history_pos - 1 >= 0 {
                            history_pos -= 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len())?;
                            input_buffer = history[history_pos as usize].clone(); 
                            self.term.write(input_buffer.as_bytes())?;
                            position = input_buffer.len();
                        } else if history_pos - 1 == -1 {
                            history_pos -= 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len())?;
                            input_buffer = String::new();
                            self.term.write(input_buffer.as_bytes())?;
                            position = 0;
                        }
                    },
                    Key::Enter => {
                        self.term.write_line("")?;
                        break;
                    },
                    _ => {}
                }
            }
            if input_buffer.is_empty() {
                continue;
            }
            history = history.into_iter().filter(|x| x != &input_buffer).collect();
            history.insert(0, input_buffer.clone());
            let output = (self.message_handler)(input_buffer, &mut self.global_state);
            match output {
                Ok(s) => {
                    match s {
                        Action::Print(m) => {
                            let output_line_split = m.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
                            for i in output_line_split {
                                self.term.write_line(&format!("{}{}", self.output_prefix, i))?;
                            }
                        },
                        Action::Exec(e) => {
                            match e {
                                Exec::Exit => {
                                    self.term.clear_screen()?;
                                    return Ok(());
                                },
                                Exec::Clear => {
                                    self.term.clear_screen()?;
                                }
                            }
                        }
                    } 
                },
                Err(s) => {  
                    let output_line_split = s.message.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
                    for i in output_line_split {
                        self.term.write_line(&format!("{}{}", self.output_prefix, style(i).red().bold()))?;
                    }
                }
            }
        }
    }
}
