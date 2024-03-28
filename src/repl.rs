use std::{any::Any, error::Error, io::{stdin, stdout, Write}};

pub enum IO {
    Output,
    Input
}

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
    messages: Vec<(IO, String)>,
    input_prefix: String,
    output_prefix: String,
    pub message_handler: F,
    pub global_state: T
}

impl<T: Any + Clone, F: FnMut(String, &mut T) -> Result<Action, HandlerError>> Repl<T, F> { 
    pub fn new(input_prefix: String, output_prefix: String, initial_state: T, handler: F) -> Repl<T, F> {
        Repl {
            messages: vec![],
            input_prefix,
            output_prefix,
            message_handler: handler,
            global_state: initial_state
        }
    }
    pub fn run_repl(&mut self) -> Result<(), Box<dyn Error>> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut stdout = stdout();
        let stdin = stdin();
        let mut input_buffer;
        loop {
            print!("{}", self.input_prefix);
            stdout.flush()?;
            input_buffer = String::new();
            stdin.read_line(&mut input_buffer)?;
            self.messages.push((IO::Input, input_buffer.clone()));
            let output = (self.message_handler)(input_buffer[0..input_buffer.len()-1].to_string(), &mut self.global_state);
            match output {
                Ok(s) => {
                    match s {
                        Action::Print(m) => {
                            self.messages.push((IO::Output, m.clone()));
                            let output_line_split = m.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
                            for i in output_line_split {
                                println!("{}{}", self.output_prefix, i);
                            }
                        },
                        Action::Exec(e) => {
                            match e {
                                Exec::Exit => {
                                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                                    return Ok(());
                                },
                                Exec::Clear => {
                                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                                }
                            }
                        }
                    } 
                },
                Err(s) => {  
                    self.messages.push((IO::Output, s.message.clone()));
                    let output_line_split = s.message.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
                    for i in output_line_split {
                        println!("{}\x1b[31;1m{}\x1b[0m", self.output_prefix, i);
                    }
                }
            }
        }
    }
}
