use std::collections::HashMap;
use lasso::{Rodeo, Spur};
use crate::types::*;

pub struct Compiler {
    ref_env: HashMap<Spur, String>,
    rodeo: Rodeo,
    functions: Vec<String>
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ref_env: HashMap::new(),
            rodeo: Rodeo::default(),
            functions: Vec::new()
        }
    }

    pub fn compile(&mut self, prog: Program) -> anyhow::Result<String> {
        let code = self.compile_program(prog, "main", vec![])?;

        let functions = self.functions.join("\n");

        Ok(format!("{}\n{}", functions, code))
    }

    pub fn compile_program(&mut self, prog: Program, fn_name: impl AsRef<str>, args: Vec<String>) -> anyhow::Result<String> {

        let mut program = format!(
            "double {}({}){{\n",
            fn_name.as_ref(),
            args.into_iter().map(|s| format!("{} {}", if fn_name.as_ref() == "main" { "int" } else { "double" }, s)).collect::<Vec<_>>().join(",")
        );

        for (_i, stmt) in prog.into_iter().enumerate() {
            program.push('\t');
            match stmt {
                Stmt::Expression(expr) => program.push_str(&format!("printf(\"%d\\n\", {})", self.compile_expr(expr)?)),
                Stmt::FunctionDefinition{ name, args, body } => {
                    let f = self.compile_program(body, name, args)?;
                    self.functions.push(f);
                },
                _ => unimplemented!()
            };

            program.push_str(";\n");
        }

        program.push_str("}\n");

        Ok(program)
    }

    fn compile_expr(&self, expr: Box<Expr>) -> anyhow::Result<String> {
        Ok(match *expr {
            Expr::Number(n) => n.to_string(),
            Expr::Ident(ident) => {
                let ident_key = self.rodeo.get(ident).ok_or(anyhow::anyhow!("variable must exist"))?;
                self.ref_env.get(&ident_key).unwrap().clone()
            }
            Expr::Call{ name, args } => {
                let ident_key = self.rodeo.get(name).ok_or(anyhow::anyhow!("function must be defined"))?;
                format!(
                    "{}({})",
                    self.ref_env.get(&ident_key).unwrap().clone(),
                    args
                        .into_iter()
                        .map(|a| self.compile_expr(a))
                        .collect::<Result<Vec<_>, anyhow::Error>>()?
                        .join(",")
                )
            },
            Expr::Add(lhs, rhs) => format!("({}+{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
            Expr::Sub(lhs, rhs) => format!("({}-{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
            Expr::Mul(lhs, rhs) => format!("({}*{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
            Expr::Div(lhs, rhs) => format!("({}/{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
            Expr::Pow(lhs, rhs) => format!("pow({},{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
        })
    }
}
