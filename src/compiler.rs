use std::collections::HashMap;
use crate::types::*;

pub fn compile(prog: Program) -> anyhow::Result<()> {

    let mut program = String::from("#include <stdio.h>\n\nint main(){\n");

    //let mut ref_env = Box::new(HashMap::new());

    for stmt in prog {
        program.push('\t');
        program.push_str(
            &match stmt {
                Stmt::Expression(expr) => format!("printf(\"%d\", {})", compile_expr(expr)?),
                _ => unimplemented!()
            }
        );
        program.push_str(";\n");
    }

    program.push('}');

    print!("{}", program);

    Ok(())
}

fn compile_expr(expr: Box<Expr>) -> anyhow::Result<String> {
    Ok(match *expr {
        Expr::Number(n) => n.to_string(),
        Expr::Ident(ident) => unimplemented!(),
        Expr::Call{ name, args } => unimplemented!(),
        Expr::Add(lhs, rhs) => format!("{}+{}", compile_expr(lhs)?, compile_expr(rhs)?),
        Expr::Sub(lhs, rhs) => format!("{}-{}", compile_expr(lhs)?, compile_expr(rhs)?),
        Expr::Mul(lhs, rhs) => format!("{}*{}", compile_expr(lhs)?, compile_expr(rhs)?),
        Expr::Div(lhs, rhs) => format!("{}/{}", compile_expr(lhs)?, compile_expr(rhs)?),
        Expr::Pow(lhs, rhs) => format!("pow({},{})", compile_expr(lhs)?, compile_expr(rhs)?),
    })
}
