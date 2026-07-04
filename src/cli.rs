use math_utils_lib::{Context, Number, Step, eval, parse};

use math_utils_lib::{ExportType, export_history};

pub fn handle_expressions<N: Number>(expressions: Vec<String>) {
    let mut context: Context<N> = Context::default();
    let mut history = vec![];

    for expression in expressions {
        #[cfg(feature = "export")]
        if expression.to_uppercase() == "EXPORT" || expression.to_uppercase() == "EXPORT --PDF" {
            let pdf = export_history(history.clone(), ExportType::Pdf);

            if let Ok(binary) = pdf {
                if let Ok(_) = std::fs::write("export.pdf", binary) {} else {
                    println!("Error exporting pdf!");
                }
            } else {
                println!("Error exporting pdf!");
            }

            println!("Exported to export.pdf!");

            continue;
        }
        #[cfg(feature = "export")]
        if expression.to_uppercase() == "EXPORT --TEX" {
            let pdf = export_history(history.clone(), ExportType::Tex);

            if let Ok(binary) = pdf {
                if let Ok(_) = std::fs::write("export.tex", binary) {} else {
                    println!("Error exporting tex!");
                }
            } else {
                println!("Error exporting tex!");
            }

            println!("Exported to export.tex!");

            continue;
        }
        #[cfg(not(feature = "export"))]
        if expression.to_uppercase() == "EXPORT" || expression.to_uppercase() == "EXPORT --TEX" {
            let pdf = export_history(history.clone(), ExportType::Tex);

            if let Ok(binary) = pdf {
                if let Ok(_) = std::fs::write("export.tex", binary) {} else {
                    println!("Error exporting tex!");
                }
            } else {
                println!("Error exporting tex!");
            }

            println!("Exported to export.tex!");

            continue;
        }
        #[cfg(feature = "export")]
        if expression.to_uppercase() == "EXPORT --PNG" {
            let pdf = export_history(history.clone(), ExportType::Png);

            if let Ok(binary) = pdf {
                if let Ok(_) = std::fs::write("export.png", binary) {} else {
                    println!("Error exporting png!");
                }
            } else {
                println!("Error exporting png!");
            }

            println!("Exported to export.png!");

            continue;
        }

        let parsed_expression = match parse(expression) {
            Ok(ast) => ast,
            Err(e) => {println!("{}", e); continue;}
        };

        let result = match eval(&parsed_expression, &mut context) {
            Ok(result) => result,
            Err(e) => {println!("{}", e); continue;}
        };

        let step = Step::new(parsed_expression, result);

        println!("{}", step.to_string());

        history.push(step);
    }
}
