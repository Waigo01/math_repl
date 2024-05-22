use math_utils_lib::{eval, export, solve, parse, ExportType, MathLibError, StepType, Value, Variable};

use crate::repl::{Action, Exec, HandlerError};

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

fn prepare_and_calc_expr(mut expr: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<Value, MathLibError> {
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();
    let parsed = parse(expr)?;
    let res = eval(&parsed, &global_state.0)?;

    global_state.1.push(StepType::Calc((parsed, res.clone(), None)));

    return Ok(res)
}

fn calc_expr(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    let res = prepare_and_calc_expr(msg, global_state)?;
    let output_msg = res.pretty_print(None);
    return Ok(output_msg);
}

fn save_calc_expr(msg: String, var: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    if !check_var_name(var.clone()) {
        return Err(MathLibError::Other("Invalid Variable Name!".to_string()));
    }
    let res = prepare_and_calc_expr(msg, global_state)?; 
    let mut found = false;
    for i in 0..global_state.0.len() {
        if global_state.0[i].name == var {
            global_state.0[i].value = res.clone();
            found = true;
            break;
        }
    }
    if !found {
        global_state.0.push(Variable { name: var.clone(), value: res.clone() });
    }
    let output_msg = res.pretty_print(Some(var.clone()));
    return Ok(output_msg);
}

fn prepare_and_solve_eq(mut expr: String, global_state: &mut (Vec<Variable>, Vec<StepType>), variable: Option<String>) -> Result<Vec<Value>, MathLibError> { 
    expr = expr.trim().split(" ").filter(|s| !s.is_empty()).collect();

    let mut equations = vec![];
    let mut parenths_open = 0;
    let mut buffer = String::new();

    for i in expr.chars() {
        if parenths_open == 0 && i == ',' {
            equations.push(buffer.clone());
            buffer.clear();
        } else {
            buffer.push(i);
        }

        if i == '(' || i == '[' || i == '{' {
            parenths_open += 1;
        } else if i == ')' || i == ']' || i == '}' {
            parenths_open -= 1;
        }
    }
    equations.push(buffer);

    let mut parsed_equations = vec![];

    for i in equations {
        if !expr.contains("=") {
            return Err(MathLibError::Other("No equal in equation!".to_string()));
        }

        let left = i.split("=").nth(0).unwrap().to_string();
        let right = i.split("=").nth(1).unwrap().to_string();

        let left_b;
        let right_b;
        if left.len() >= right.len() {
            left_b = parse(left)?;
            right_b = parse(right)?;
        } else {
            left_b = parse(right)?;
            right_b = parse(left)?;
        }

        parsed_equations.push((left_b, right_b));
    }

    let roots = solve(parsed_equations.clone(), &global_state.0.clone())?;

    global_state.1.push(StepType::Equ((parsed_equations, roots.clone(), variable)));
    
    Ok(roots)
}

pub fn solve_eq(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    let roots = prepare_and_solve_eq(msg, global_state, None)?;

    let output_string;

    if roots.len() == 0 {
        output_string = "No solutions found!".to_string();
    } else if roots.len() == 1 {
        output_string = roots[0].pretty_print(Some("x".to_string()));
    } else {
        output_string = roots.iter().enumerate().map(|(i, x)| x.pretty_print(Some(format!("x_{}", i)))).collect::<Vec<String>>().join("\n");
    }

    return Ok(output_string);
}

fn save_solved_eq(msg: String, var: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<String, MathLibError> {
    if !check_var_name(var.clone()) {
        return Err(MathLibError::Other("Invalid Variable Name!".to_string()));
    }

    let roots = prepare_and_solve_eq(msg, global_state, Some(var.clone()))?;
    
    global_state.0 = global_state.0.clone().into_iter().filter(|x| x.name != var).collect();

    let output_string;

    if roots.len() == 0 {
        output_string = "No solutions found!".to_string();
    } else if roots.len() == 1 {
        output_string = roots[0].pretty_print(Some(var.clone()));
    } else {
        output_string = roots.iter().enumerate().map(|(i, x)| x.pretty_print(Some(format!("{}_{}", var, i)))).collect::<Vec<String>>().join("\n");
    }

    if roots.len() == 1 {
        global_state.0.push(Variable { name: var.clone(), value: roots[0].clone() });
    } else {
        for i in 0..roots.len() {
            global_state.0.push(Variable {
                name: format!("{}_{}", var, i),
                value: roots[i].clone()
            })
        }
    }

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

pub fn handle_message(msg: String, global_state: &mut (Vec<Variable>, Vec<StepType>)) -> Result<Action, HandlerError> {
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "VARS" {
        let output_buffer = global_state.0.iter().map(|x| x.value.pretty_print(Some(x.name.clone()))).collect::<Vec<String>>().join("\n");
        return Ok(Action::Print(output_buffer))
    }
    if msg.len() == 5 && msg[0..=4].to_string().to_uppercase() == "CLEAR" {
        global_state.0 = vec![
            Variable {
                name: "pi".to_string(),
                value: Value::Scalar(std::f64::consts::PI)
            },
            Variable {
                name: "e".to_string(),
                value: Value::Scalar(std::f64::consts::E)
            }
        ];
        global_state.1.clear();
        return Ok(Action::Exec(Exec::Clear))
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "EXIT" {
        return Ok(Action::Exec(Exec::Exit));
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "HELP" {
        return Ok(Action::Print(HELP_MESSAGE.to_string()));
    }
    if msg.split(" ").nth(0).unwrap().len() == 6 && msg[0..=5].to_string().to_uppercase() == "EXPORT" {
        match msg.to_lowercase().as_str() {
            "export" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Pdf); 

                return Ok(Action::Print("Exported to export.pdf!".to_string()))
            },
            "export --tex" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Tex);
                
                return Ok(Action::Print("Exported to export.tex!".to_string()))
            },
            "export --png" => {
                export(global_state.1.clone(), "export".to_string(), ExportType::Png); 

                return Ok(Action::Print("Exported to export.png!".to_string()))
            },
            _ => {return Ok(Action::Print("Please use export/export --tex or export --png!".to_string()))}
        }
    }
    if msg.len() == 9 && msg[0..=8].to_string().to_uppercase() == "CLEARVARS" {
        global_state.0 = vec![
            Variable {
                name: "pi".to_string(),
                value: Value::Scalar(std::f64::consts::PI)
            },
            Variable {
                name: "e".to_string(),
                value: Value::Scalar(std::f64::consts::E)
            }
        ];
        let output_buffer = global_state.0.iter().map(|x| x.value.pretty_print(Some(x.name.clone()))).collect::<Vec<String>>().join("\n");

        return Ok(Action::Print(output_buffer))
    }

    let expression: String = msg.trim().split(" ").filter(|s| !s.is_empty()).collect();
    let split_expression: Option<(String, String)> = expression.split_once("=").map(|x| (x.0.to_string(), x.1.to_string()));

    let msg;

    if split_expression.is_none() {
        msg = calc_expr(expression, global_state)?;
        return Ok(Action::Print(msg))
    } else {
        let split_expression_unwraped = split_expression.unwrap();
        if split_expression_unwraped.1.starts_with("eq") {
            msg = save_solved_eq(split_expression_unwraped.1[2..].to_string(), split_expression_unwraped.0, global_state)?;
            return Ok(Action::Print(msg))
        } else if !split_expression_unwraped.0.starts_with("eq") {
            msg = save_calc_expr(split_expression_unwraped.1, split_expression_unwraped.0, global_state)?;
            return Ok(Action::Print(msg))
        } else {
            msg = solve_eq(expression[2..].to_string(), global_state)?;
            return Ok(Action::Print(msg))
        }
    }
}
