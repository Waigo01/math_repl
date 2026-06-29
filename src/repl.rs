use std::{error::Error, io::Write, time::Duration};

use console::{Key, Term, TermFamily, style};
use math_utils_lib::{Context, MathLibError, Number, Step};

use crate::message_handler::LocalLatexError;

pub enum Exec {
    Exit,
    Clear
}

pub enum Action {
    Print(String),
    Exec(Exec),
    Tutorial
}

pub struct State<N: Number> {
    pub context: Context<N>,
    pub history: Vec<Step<N>>
}

impl<N: Number> State<N> {
    pub fn new(context: Context<N>) -> Self {
        return State { context, history: vec![] };
    }
}

pub struct HandlerError {
    pub message: String
}

impl<E: Into<MathLibError>> From<E> for HandlerError {
    fn from(value: E) -> Self {
        HandlerError { message: value.into().get_reason() }
    }
}

impl From<LocalLatexError> for HandlerError {
    fn from(value: LocalLatexError) -> Self {
        HandlerError { message: value.get_reason() }
    }
}

#[cfg(feature = "export")]
const REPL_EXAMPLES: [(&'static str, &'static str); 17] = [("You can do the most basic of calculations: ", "3*3"), ("You can also create variables: ", "a=3"), ("And then do calculations with those variables: ", "3a"), ("You can also save matrices to variables: ", "M = [[3, 4, 5], [1, 2, 3], [5, 6, 7]]"), ("And do some calculations with them: ", "3*M"), ("Vectors are also supported: ", "B = [2, 3, 4]"), ("As is linear algebra: ", "M*B"), ("You can even create custom functions with one or multiple variables as inputs: ", "f(x) = 5x^2+2x+x"), ("There is also support for lists of values. This will evaluate the function f at both 5 and 10: ", "f({5, 10})"), ("There is even an equation solver. The inputs can be read as 'solve equation x^2=9 in terms of x': ", "eq(x^2=9, x)"), ("This equation solver can also solve systems of equations: ", "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)"), ("You can also do some boolean operations: ", "3==3 & 2<4"), ("Then you can create functions with case distinctions: ", "relu(x) = if(x < 0, 0, x)"), ("And you can export the steps to a pdf: ", "export"), ("There are also several internal commands, such as vars to display variables: ", "vars"), ("You can also clear all variables: ", "clearvars"), ("You can also clear the repl: ", "clear")];

#[cfg(not(feature = "export"))]
const REPL_EXAMPLES: [(&'static str, &'static str); 16] = [("You can do the most basic of calculations: ", "3*3"), ("You can also create variables: ", "a=3"), ("And then do calculations with those variables: ", "3a"), ("You can also save matrices to variables: ", "M = [[3, 4, 5], [1, 2, 3], [5, 6, 7]]"), ("And do some calculations with them: ", "3*M"), ("Vectors are also supported: ", "B = [2, 3, 4]"), ("As is linear algebra: ", "M*B"), ("You can even create custom functions with one or multiple variables as inputs: ", "f(x) = 5x^2+2x+x"), ("There is also support for lists of values. This will evaluate the function f at both 5 and 10: ", "f({5, 10})"), ("There is even an equation solver. The inputs can be read as 'solve equation x^2=9 in terms of x': ", "eq(x^2=9, x)"), ("This equation solver can also solve systems of equations: ", "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)"), ("You can also do some boolean operations: ", "3==3 & 2<4"), ("Then you can create functions with case distinctions: ", "relu(x) = if(x < 0, 0, x)"), ("There are also several internal commands, such as vars to display variables: ", "vars"), ("You can also clear all variables: ", "clearvars"), ("You can also clear the repl: ", "clear")];

pub struct Repl<N: Number, F: FnMut(String, &mut State<N>, i32, bool, String) -> Result<Action, HandlerError>> {
    term: Term,
    input_prefix: String,
    output_prefix: String,
    pub message_handler: F,
    pub global_state: State<N>
}

impl<N: Number, F: FnMut(String, &mut State<N>, i32, bool, String) -> Result<Action, HandlerError>> Repl<N, F> {
    /// used to initialize a new [Repl].
    pub fn new(input_prefix: String, output_prefix: String, initial_state: State<N>, handler: F) -> Repl<N, F> {
        Repl {
            term: Term::stdout(),
            input_prefix,
            output_prefix,
            message_handler: handler,
            global_state: initial_state
        }
    }
    fn read_escape_code<S: Into<String>>(&self, code: S) -> Result<String, Box<dyn Error>> {
        self.term.write_line(&code.into())?;
        let mut escape_return = String::new();
        while let Ok(key) = self.term.read_key_raw() {
            match key {
                Key::Char(char) if char == 'c' => break,
                Key::Char(char) => {
                    escape_return.push(char);
                },
                Key::UnknownEscSeq(_) => {
                    escape_return.push('\\');
                },
                _ => break
            }
        }
        Ok(escape_return)
    }
    fn write_char_by_char<S: Into<String>>(&mut self, input: S) -> Result<(), Box<dyn Error>> {
        for char in input.into().chars() {
            self.term.write(char.to_string().as_bytes())?;
            std::thread::sleep(Duration::from_millis(30));
        }

        Ok(())
    }

    pub fn run_repl(&mut self) -> Result<(), Box<dyn Error>> {
        self.term.set_title("math_repl");
        
        let mut history: Vec<String> = vec![];

        let features = self.term.features();
        let cell_height;
        let use_kitty;
        let mut fg_color = "#FFFFFF".to_string();

        let escape_return = if !features.is_msys_tty() && features.family() == TermFamily::UnixTerm {
            self.read_escape_code("\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\\x1b[c")?
        } else {
            String::new()
        };


        if escape_return.contains("OK") {
            let escape_return = self.read_escape_code("\x1b[16t\x1b[c")?;

            let mut cell_height_split = escape_return.split(";");
            cell_height = if cell_height_split.clone().count() != 0 && let Some(height) = cell_height_split.nth(0) && height.len() >= 2 && let Ok(parsed_height) = height[1..].parse::<i32>() {
                parsed_height
            } else {
                0
            };

            use_kitty = cell_height != 0;
            if use_kitty {
                self.term.write_line("\x1b_Ga=d\x1b\\")?;
            }

            let escape_return = self.read_escape_code("\x1b]10;?\x07\x1b[c")?;

            if escape_return.contains("rgb:") {
                let split = escape_return.split("rgb:").nth(1).unwrap();
                let split = split.split("\\").nth(0).unwrap();

                let mut split = split.split("/");

                let mut hex = "#".to_string();

                while let Some(c) = split.next() {
                    hex += &c[0..2];
                }

                if hex.len() == 7 {
                    fg_color = hex;
                }
            }
        } else {
            cell_height = 0;
            use_kitty = false;
        }

        self.term.clear_screen()?;

        let mut tutorial: Option<usize> = None;

        loop {
            let mut input_buffer = String::new();
            if let Some(example_step) = tutorial && example_step < REPL_EXAMPLES.len() {
                self.term.write(self.output_prefix.as_bytes())?;
                self.term.flush()?;
                self.write_char_by_char(REPL_EXAMPLES[example_step].0)?;
                self.term.write_line("")?;

                self.term.write(self.input_prefix.as_bytes())?;
                self.term.flush()?;

                self.term.write(REPL_EXAMPLES[example_step].1.as_bytes())?;
                input_buffer = REPL_EXAMPLES[example_step].1.to_string();
            } else if tutorial.is_some() {
                tutorial = None;
                self.term.write(self.input_prefix.as_bytes())?;
                self.term.flush()?;
            } else {
                self.term.write(self.input_prefix.as_bytes())?;
                self.term.flush()?;
            }
            let mut position = 0;
            let mut history_pos = -1;
            loop {
                match self.term.read_key()? {
                    Key::Char(c) if tutorial.is_none() => {
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
                    Key::ArrowLeft if tutorial.is_none() => {
                        if position as i32-1 >= 0 {
                            self.term.move_cursor_left(1)?;
                            position -= 1;
                        } 
                    },
                    Key::ArrowRight if tutorial.is_none() => {
                        if position+1 <= input_buffer.len() {
                            self.term.move_cursor_right(1)?;
                            position += 1;
                        }
                    },
                    Key::Backspace if tutorial.is_none() => {
                        if position as i32-1 >= 0 {
                            input_buffer.remove(position-1);
                            position -= 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len()+1)?;
                            self.term.write(input_buffer.as_bytes())?;
                            self.term.move_cursor_left(input_buffer.len()-position)?;
                        } 
                    },
                    Key::ArrowUp if tutorial.is_none() => {
                        if history_pos + 1 < history.len() as i32 && history_pos + 1 >= 0 {
                            history_pos += 1;
                            self.term.move_cursor_right(input_buffer.len()-position)?;
                            self.term.clear_chars(input_buffer.len())?;
                            input_buffer = history[history_pos as usize].clone(); 
                            self.term.write(input_buffer.as_bytes())?;
                            position = input_buffer.len();
                        }
                    },
                    Key::ArrowDown if tutorial.is_none() => {
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
            let output = (self.message_handler)(input_buffer, &mut self.global_state, cell_height, use_kitty, fg_color.clone());
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
                                    if use_kitty {
                                        self.term.write_line("\x1b_Ga=d\x1b\\")?;
                                    }
                                    self.term.clear_screen()?;
                                    return Ok(());
                                },
                                Exec::Clear => {
                                    if use_kitty {
                                        self.term.write_line("\x1b_Ga=d\x1b\\")?;
                                    }
                                    self.term.clear_screen()?;
                                }
                            }
                        },
                        Action::Tutorial => {
                            tutorial = Some(0);
                            continue;
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
            if let Some(example_step) = tutorial.as_mut() {
                *example_step += 1;
            }
        }
    }
}
