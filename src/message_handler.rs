use math_utils_lib::{eval, export, find_roots, parse, parser::{Binary, SimpleOpType, Operation}, ExportType, MathLibError, StepType, Value, Variable};

use crate::repl::{Exec, Action, HandlerError};

fn check_var_name(var: String) -> bool {
    let var_chars: Vec<char> = var.chars().collect();
    if !var_chars[0].is_alphabetic() && var_chars[0] != '\\' {
        return false;
    }
    let mut parenths_open = 0;
    let mut previous_char = '\\';
    for i in var_chars {
        if i == '{' {
            parenths_open += 1;
        }
        if i == '}' {
            parenths_open -= 1;
        }
        if i.is_numeric() && parenths_open == 0 && previous_char != '_' {
            return false;
        }
        previous_char = i;
    }
    return true;
}

impl From<MathLibError> for HandlerError {
    fn from(value: MathLibError) -> Self {
        return HandlerError { message: value.get_reason() }
    }
}

fn calc_expr(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    let parsed = parse(msg)?;
    let res = eval(&parsed, &global_state.0)?;
    let output_msg = res.pretty_print(None);
    global_state.1.push(StepType::Calc((parsed, res, None)));
    return Ok(output_msg);
}

fn save_calc_expr(msg: String, var: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    if !check_var_name(var.clone()) {
        return Err(MathLibError::Other("Invalid Variable Name!".to_string()));
    }
    let parsed = parse(msg)?;
    let res = eval(&parsed, &global_state.0)?;
    let mut found = false;
    for i in 0..global_state.0.len() {
        if global_state.0[i].name == var {
            global_state.0[i].value = res.clone();
            found = true;
            break;
        }
    }
    if !found {
        global_state.0.push(Variable::new(var.clone(), res.clone()));
    }
    let output_msg = res.pretty_print(Some(var.clone()));
    global_state.1.push(StepType::Calc((parsed, res, Some(var))));
    return Ok(output_msg);
}

fn solve_eq(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    let left = msg.split("=").nth(0).unwrap().to_string();
    let right = msg.split("=").nth(1).unwrap().to_string();

    let left_b;
    let right_b;
    if left.len() >= right.len() {
        left_b = parse(left)?;
        right_b = parse(right)?;
    } else {
        left_b = parse(right)?;
        right_b = parse(left)?;
    }

    let root_b = Binary::from_operation(Operation::SimpleOperation {
        op_type: SimpleOpType::Sub,
        left: left_b.clone(),
        right: right_b.clone()
    });

    let roots = find_roots(root_b, global_state.clone().0, "x".to_string())?;

    let output_string;

    if roots.len() == 0 {
        output_string = "No solutions found!".to_string();
    } else if roots.len() == 1 {
        output_string = roots[0].pretty_print(Some("x".to_string()));
    } else {
        output_string = roots.iter().enumerate().map(|(i, x)| x.pretty_print(Some(format!("x_{}", i)))).collect::<Vec<String>>().join("\n");
    }

    global_state.1.push(StepType::Equ((left_b, right_b, roots, None)));

    return Ok(output_string);
}

fn save_solved_eq(msg: String, var: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    if !check_var_name(var.clone()) {
        return Err(MathLibError::Other("Invalid Variable Name!".to_string()));
    }
    let left = msg.split("=").nth(0).unwrap().to_string();
    let right = msg.split("=").nth(1).unwrap().to_string();

    let left_b;
    let right_b;
    if left.len() >= right.len() {
        left_b = parse(left)?;
        right_b = parse(right)?;
    } else {
        left_b = parse(right)?;
        right_b = parse(left)?;
    }

    let root_b = Binary::from_operation(Operation::SimpleOperation {
        op_type: SimpleOpType::Sub,
        left: left_b.clone(),
        right: right_b.clone()
    });

    global_state.0 = global_state.0.clone().into_iter().filter(|x| x.name != var).collect();

    let roots = find_roots(root_b, global_state.clone().0, var.clone())?;

    let output_string;

    if roots.len() == 0 {
        output_string = "No solutions found!".to_string();
    } else if roots.len() == 1 {
        output_string = roots[0].pretty_print(Some(var.clone()));
    } else {
        output_string = roots.iter().enumerate().map(|(i, x)| x.pretty_print(Some(format!("{}_{}", var, i)))).collect::<Vec<String>>().join("\n");
    }

    if roots.len() == 1 {
        global_state.0.push(Variable::new(var.clone(), roots[0].clone()));
    } else {
        for i in 0..roots.len() {
            global_state.0.push(Variable::new(format!("{}_{}", var, i), roots[i].clone()));
        }
    }

    global_state.1.push(StepType::Equ((left_b, right_b, roots, Some(var))));

    return Ok(output_string);
}

const HELP_MESSAGE: &str = "You can do 4 basic operations:
        Calculate something: <expr>
        Save the results of a calculation to a variable: <varName> = <expr>
        Solve an equation (using x as variable to solve for): eq <expr> = <expr>
        Solve an equation and save it into a variable (using <varName> as variable to solve for): <varName> = eq <expr> = <expr>
    As an <expr> counts:
        A scalar (number): <number>
        A vector: [<1>, <2>, ..., <n>]
        A matrix: [[<1:1>, <1:2>, ..., <1:n>], [<2:1>, <2:2>, ..., <2:n>], ..., [<n:1>, <n:2>, ..., <n:n>]]
        A Variable: Any previously defined variable.

        You can also use all common operations (see https://docs.rs/math_utils_lib/latest/math_utils_lib/parser/enum.OpType.html)
        between all different types (It will tell you, when it can't calculate something).
    Additional commands:
        clear: Clears the screen, the history for LaTeX export and all vars except pi and e.
        clearvars: Clears all vars except pi and e.
        vars: Displays all vars.
        export (< --tex | --png >): Exports history since last clear in specified format (leave blank for .pdf).
        help: This help page.
        exit: Exits the REPL.
    Some rules:
        Variable Names must start with an alphabetical letter or a \\. (Greek symbols in LaTeX style get replaced before printing).
        Numbers in Variable Names are only allowed in LaTeX style subscript.
        Any other rules will be explained to you in a (not so) nice manner by the program."; 

/// describes the main message handler for math_repl.
pub fn handle_message(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<Action, HandlerError> {
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "VARS" {
        let output_buffer = global_state.0.iter().map(|x| x.value.pretty_print(Some(x.name.clone()))).collect::<Vec<String>>().join("\n"); 
        return Ok(Action::Print(output_buffer));
    }
    if msg.len() == 5 && msg[0..=4].to_string().to_uppercase() == "CLEAR" {
        global_state.0 = vec![
            Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI)),
            Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E))
        ];
        global_state.1.clear();
        return Ok(Action::Exec(Exec::Clear));
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "EXIT" {
        return Ok(Action::Exec(Exec::Exit))
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "HELP" {
        return Ok(Action::Print(HELP_MESSAGE.to_string()));
    }
    if msg.split(" ").nth(0).unwrap().len() == 6 && msg[0..=5].to_string().to_uppercase() == "EXPORT" {
        match msg.to_lowercase().as_str() {
            "export" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Pdf);
                return Ok(Action::Print("Exported to export.pdf!".to_string()));
            },
            "export --tex" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Tex);
                return Ok(Action::Print("Exported to export.tex!".to_string()));
            },
            "export --png" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Png);
                return Ok(Action::Print("Exported to export-[...].png".to_string()));
            },
            _ => {return Err(HandlerError{message: "Export in formats: pdf(), tex(--tex), png(--png)!".to_string()})}
        }
    }
    if msg.len() == 9 && msg[0..=8].to_string().to_uppercase() == "CLEARVARS" {
        global_state.0 = vec![
            Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI)),
            Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E))
        ];
        let output_buffer = global_state.0.iter().map(|x| x.value.pretty_print(Some(x.name.clone()))).collect::<Vec<String>>().join("\n"); 
        return Ok(Action::Print(output_buffer));
    }

    let expression: String = msg.trim().split(" ").filter(|s| !s.is_empty()).collect();
    let split_expression: Vec<String> = expression.split("=").map(|x| x.to_string()).collect();

    if split_expression.len() == 1 {
        let msg = calc_expr(expression, global_state)?;
        return Ok(Action::Print(msg));
    } else if split_expression.len() == 2 {
        if split_expression[0].starts_with("eq") {
            let msg = solve_eq(format!("{}={}", split_expression[0][2..].to_string(), split_expression[1]), global_state)?;
            return Ok(Action::Print(msg));
        } else {
            let msg = save_calc_expr(split_expression[1].clone(), split_expression[0].clone(), global_state)?;
            return Ok(Action::Print(msg));
        }

    } else if split_expression.len() == 3 {
        if split_expression[1].starts_with("eq") {
            let msg = save_solved_eq(format!("{}={}", split_expression[1][2..].to_string(), split_expression[2]), split_expression[0].clone(), global_state)?;
            return Ok(Action::Print(msg));
        } else {
            return Err(HandlerError{message: "What are you trying to do?".to_string()});
        }
    } else {
        return Err(HandlerError{message: "What are you trying to do?".to_string()});
    }
}
