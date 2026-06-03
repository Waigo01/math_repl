use math_utils_lib::{Context, ExportType, Step, errors::LatexError, eval, export_history, parse, svg_from_latex};

use base64::prelude::*;

use crate::repl::{Action, Exec, HandlerError, State};

use resvg::{render, tiny_skia::Pixmap, usvg::{Options, Transform, Tree}};

const BASE_SVG_HEIGHT: f32 = 10.;

pub fn png_from_latex<S: Into<String>>(latex: String, line_color: S, height: i32) -> Result<(Vec<u8>, u32), LatexError> {
    let svg = svg_from_latex(latex, line_color)?;

    let tree = Tree::from_str(&svg, &Options::default())?;

    let height_scale = height as f32/BASE_SVG_HEIGHT*0.65;

    let height = tree.size().height() * height_scale;

    let dest_width = ((tree.size().width()/tree.size().height()) * height as f32).ceil();
    let width_scale = dest_width/tree.size().width();

    let mut pixmap = Pixmap::new(dest_width as u32, height as u32).unwrap();

    render(&tree, Transform::from_row(width_scale, 0., 0., height_scale, 0., 0.), &mut pixmap.as_mut());

    Ok((pixmap.encode_png().ok().unwrap(), pixmap.height()))
}

pub fn print_latex_kitty(latex: String, color: String, cell_height: i32) -> Result<String, HandlerError> {

    let (png, height) = png_from_latex(latex, color, cell_height)?;

    let mut temp_dir = std::env::temp_dir();

    temp_dir.push("tty-graphics-protocol.png");

    let _ = std::fs::write(&temp_dir, png);

    let base64_encoded = BASE64_STANDARD.encode(temp_dir.to_str().unwrap());

    let command = format!("\x1b_Gf=100,t=t,a=T,Y={},C=1;{base64_encoded};\x1b\\", cell_height/2+3);

    let n_newlines = (height as f32/cell_height as f32).ceil() as i32;

    return Ok(format!("{command}{}", (0..n_newlines).map(|_| "\n".to_string()).collect::<Vec<String>>().join("")));
}

pub fn handle_message(msg: String, global_state: &mut State, cell_height: i32, use_kitty: bool, foreground: String) -> Result<Action, HandlerError> {
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "VARS" {
        if use_kitty {
            let latex_vars: String = "\\begin{align}".to_string() + &global_state.context.vars.iter()
                .map(|v| v.as_latex(true))
                .collect::<Vec<String>>()
                .join(" \\\\") + " \\\\";

            let latex_funs: String = global_state.context.funs.iter()
                .map(|f| f.as_latex(true))
                .collect::<Vec<String>>()
                .join(" \\\\") + "\\end{align}";

            let output = print_latex_kitty(latex_vars + &latex_funs, foreground, cell_height)?;

            return Ok(Action::Print(output))
        } else {
            let string_vars: String = global_state.context.vars.iter()
                .map(|v| v.as_string())
                .collect::<Vec<String>>()
                .join("\n");

            let string_funs: String = global_state.context.funs.iter()
                .map(|f| f.as_string())
                .collect::<Vec<String>>()
                .join("\n");

            return Ok(Action::Print(format!("{}{}{}", string_vars, if string_funs != "" {"\n"} else {""}, string_funs)));
        }
    }
    if msg.len() == 5 && msg[0..=4].to_string().to_uppercase() == "CLEAR" {
        global_state.context = Context::default();
        global_state.history.clear();
        return Ok(Action::Exec(Exec::Clear))
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "EXIT" {
        return Ok(Action::Exec(Exec::Exit));
    }
    if msg.len() == 4 && msg[0..=3].to_string().to_uppercase() == "HELP" {
        return Ok(Action::Tutorial);
    }
    if msg.split(" ").nth(0).unwrap().len() == 6 && msg[0..=5].to_string().to_uppercase() == "EXPORT" {
        match msg.to_lowercase().as_str() {
            "export" | "export --pdf" => {
                let pdf = export_history(global_state.history.clone(), ExportType::Pdf);

                if let Ok(binary) = pdf {
                    if let Ok(_) = std::fs::write("export.pdf", binary) {} else {
                        return Err(HandlerError { message: "Error exporting pdf!".to_string() });
                    }
                } else {
                    return Err(HandlerError { message: "Error exporting pdf!".to_string() });
                }

                return Ok(Action::Print("Exported to export.pdf!".to_string()));
            },
            "export --tex" => {
                let tex = export_history(global_state.history.clone(), ExportType::Tex);

                if let Ok(binary) = tex {
                    if let Ok(_) = std::fs::write("export.tex", binary) {} else {
                        return Err(HandlerError { message: "Error exporting tex!".to_string() });
                    }
                } else {
                    return Err(HandlerError { message: "Error exporting tex!".to_string() });
                }

                return Ok(Action::Print("Exported to export.tex!".to_string()));
            },
            "export --png" => {
                let png = export_history(global_state.history.clone(), ExportType::Png);

                if let Ok(binary) = png {
                    if let Ok(_) = std::fs::write("export.png", binary) {} else {
                        return Err(HandlerError { message: "Error exporting png!".to_string() });
                    }
                } else {
                    return Err(HandlerError { message: "Error exporting png!".to_string() });
                }

                return Ok(Action::Print("Exported to export.png!".to_string()));
            },
            _ => {return Ok(Action::Print("Please use export/export --tex, export --png or export --pdf!".to_string()))}
        }
    }
    if msg.len() == 9 && msg[0..=8].to_string().to_uppercase() == "CLEARVARS" {
        global_state.context = Context::default();

        if use_kitty {
            let latex_vars: String = "\\begin{align}".to_string() + &global_state.context.vars.iter()
                .map(|v| v.as_latex(true))
                .collect::<Vec<String>>()
                .join(" \\\\") + " \\\\";

            let latex_funs: String = global_state.context.funs.iter()
                .map(|f| f.as_latex(true))
                .collect::<Vec<String>>()
                .join(" \\\\") + "\\end{align}";

            let output = print_latex_kitty(latex_vars + &latex_funs, foreground, cell_height)?;

            return Ok(Action::Print(output))
        } else {
            let string_vars: String = global_state.context.vars.iter()
                .map(|v| v.as_string())
                .collect::<Vec<String>>()
                .join("\n");

            let string_funs: String = global_state.context.funs.iter()
                .map(|f| f.as_string())
                .collect::<Vec<String>>()
                .join("\n");

            return Ok(Action::Print(format!("{}{}{}", string_vars, if string_funs != "" {"\n"} else {""}, string_funs)));
        }
    }

    let expression: String = msg.trim().split(" ").filter(|s| !s.is_empty()).collect();
    
    let parsed_expr = parse(expression)?;

    let res = eval(&parsed_expr, &mut global_state.context)?;

    let step = Step::new(parsed_expr, res);
    
    let latex = step.as_latex_inline();
    let pure_string = step.as_string();

    global_state.history.push(step);
    
    if use_kitty {
        let output = print_latex_kitty(latex, foreground, cell_height)?;

        return Ok(Action::Print(output));
    } else {
        return Ok(Action::Print(pure_string));
    }
}
