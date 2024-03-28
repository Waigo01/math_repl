use std::{any::Any, error::Error, io::Write};

use console::{style, Key, Term};

pub enum Exec {
    Exit,
    Clear
}

pub enum Action {
    Print(String),
    Exec(Exec)
}

pub struct HandlerError {
    pub message: String
}

pub struct Repl<T: Any + Clone, F: FnMut(String, &mut T) -> Result<Action, HandlerError>> {
    term: Term,
    input_prefix: String,
    output_prefix: String,
    pub message_handler: F,
    pub global_state: T
}

impl<T: Any + Clone, F: FnMut(String, &mut T) -> Result<Action, HandlerError>> Repl<T, F> { 
    pub fn new(input_prefix: String, output_prefix: String, initial_state: T, handler: F) -> Repl<T, F> {
        Repl {
            term: Term::stdout(),
            input_prefix,
            output_prefix,
            message_handler: handler,
            global_state: initial_state
        }
    }
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
                        self.term.move_cursor_right(input_buffer.len()-position+1)?;
                        self.term.clear_chars(input_buffer.len())?;
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
