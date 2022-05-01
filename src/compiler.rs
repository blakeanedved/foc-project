use crate::types::*;
use crate::utils::*;
use lasso::{Rodeo, Spur};
use std::collections::HashMap;
use std::fs::write;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

const C_HEADER: &str = r#"#include <stdio.h>
#include <math.h>

void print_number(double n){
	if (n==(long long)n) {
		printf("%lld\n", (long long)n);
	} else {
		printf("%lf\n", (double)n);
	}
}"#;

pub struct Compiler {
    ref_env: HashMap<Spur, String>,
    rodeo: Rodeo,
    functions: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ref_env: HashMap::new(),
            rodeo: Rodeo::default(),
            functions: Vec::new(),
        }
    }

    pub fn compile(prog: &Program, args: crate::Args) -> anyhow::Result<()> {
        let mut c = Compiler::new();

        let code = c.compile_program(&prog, "main", &vec![])?;

        let functions = c.functions.join("\n");

        let program = format!("{}\n{}\n{}", C_HEADER, functions, code);

        let input_filestem = Path::new(&args.filename)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        if args.intermediates {
            write(format!("{}.c", input_filestem), &program)?;
        }

        let mut cmd = Command::new("gcc");
        cmd.args(&[
            "-o",
            &if let Some(filename) = args.output {
                filename
            } else {
                input_filestem
            },
            "-x",
            "c",
            "-O3",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

        let mut child = cmd.spawn().expect("could not start gcc");

        let stdin = child.stdin.as_mut().unwrap();

        stdin.write_all(program.as_bytes())?;

        let output = child.wait_with_output()?;

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!(
                "Program compilation failed\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    pub fn compile_program(
        &mut self,
        prog: &Program,
        fn_name: impl AsRef<str>,
        args: &Vec<String>,
    ) -> anyhow::Result<String> {
        if self.rodeo.contains(fn_name.as_ref().clone()) {
            return Err(anyhow::anyhow!(
                "function {} already exists",
                fn_name.as_ref()
            ));
        }

        let mut program = if fn_name.as_ref() == "main" {
            String::from("int main(){\n")
        } else {
            let key = self.rodeo.get_or_intern(fn_name.as_ref());
            let name = generate_function_name(fn_name.as_ref());
            self.ref_env.insert(key, name.clone());

            format!(
                "double {}({}){{\n",
                name,
                args.into_iter()
                    .map(|s| format!("double {}", s))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };

        program.push_str(&self.compile_body(
            prog,
            if fn_name.as_ref() == "main" {
                false
            } else {
                true
            },
        )?);

        if fn_name.as_ref() == "main" {
            program.push_str("\treturn 0;\n");
        }

        program.push_str("}\n");

        Ok(program)
    }

    fn compile_body(&mut self, body: &Program, function_body: bool) -> anyhow::Result<String> {
        let mut program = String::new();

        let count = body.len();

        for (i, stmt) in body.into_iter().enumerate() {
            match stmt {
                Stmt::Expression(ref expr) => {
                    if i == count - 1 && function_body == true {
                        program.push_str(&format!("\treturn {};\n", self.compile_expr(expr)?));
                    } else {
                        program.push_str(&format!(
                            "\tprint_number({});\n",
                            self.compile_expr(expr)?
                        ));
                    }
                }
                Stmt::FunctionDefinition {
                    ref name,
                    ref args,
                    ref body,
                } => {
                    let f = self.compile_program(body, name, args)?;
                    self.functions.push(f);
                }
                Stmt::Declaration {
                    ref name,
                    ref value,
                } => {
                    program.push_str(&format!("\tdouble {}={};\n", name, self.compile_expr(value)?));
                }
                Stmt::Assignment {
                    ref name,
                    ref value,
                } => {
                    program.push_str(&format!("\t{}={};\n", name, self.compile_expr(value)?));
                }
                Stmt::IfStatement {
                    ref cond,
                    ref body,
                    ref branch,
                } => {
                    match branch {
                        Some(b) => program.push_str(&format!(
                            "\tif ({}){{\n{}\t}} else {{\n{}\t}}\n",
                            self.compile_expr(cond)?,
                            self.compile_body(body, false)?,
                            self.compile_body(b, false)?
                        )),
                        None => program.push_str(&format!(
                            "\tif ({}){{\n{}\t}}\n",
                            self.compile_expr(cond)?,
                            self.compile_body(body, false)?
                        )),
                    };
                }
                Stmt::For {
                    ref body,
                    ref ident,
                    ref exprs,
                } => program.push_str(&match exprs.len() {
                    1 => format!(
                        "\tfor (int {}=0;{0}<{};{0}++){{\n{}\t}}\n",
                        ident,
                        self.compile_expr(&exprs[0])?,
                        self.compile_body(body, false)?
                    ),
                    2 => format!(
                        "\tfor (int {}=(int){};{1}>{2}?(int){0}>(int){2}:(int){0}<(int){};{1}>{2}?{0}--:{0}++){{\n{}\t}}\n",
                        ident,
                        self.compile_expr(&exprs[0])?,
                        self.compile_expr(&exprs[1])?,
                        self.compile_body(body, false)?
                    ), // start>stop?i>stop:i<stop
                    3 => format!(
                        "\tfor (int {}=(int){};{1}>{2}?(int){0}>(int){2}:(int){0}<(int){};{0}+=(int){}){{\n{}\t}}\n",
                        ident,
                        self.compile_expr(&exprs[0])?,
                        self.compile_expr(&exprs[1])?,
                        self.compile_expr(&exprs[2])?,
                        self.compile_body(body, false)?
                    ),
                    _ => unreachable!(),
                }),
                Stmt::While { body, expr } => program.push_str(&format!("\twhile ({}){{\n{}\t}}\n", self.compile_expr(expr)?, self.compile_body(body, false)?)),
            }
        }

        Ok(program)
    }

    fn compile_expr(&self, expr: &Box<Expr>) -> anyhow::Result<String> {
        Ok(match (**expr).clone() {
            Expr::Number(n) => n.to_string(),
            Expr::Ident(ident) => ident,
            Expr::Call { name, args } => {
                let ident_key = self
                    .rodeo
                    .get(name)
                    .ok_or(anyhow::anyhow!("function must be defined"))?;
                format!(
                    "{}({})",
                    self.ref_env.get(&ident_key).unwrap().clone(),
                    args.into_iter()
                        .map(|a| self.compile_expr(&a))
                        .collect::<Result<Vec<_>, anyhow::Error>>()?
                        .join(",")
                )
            }
            Expr::Add(ref lhs, ref rhs) => {
                format!("({}+{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Sub(ref lhs, ref rhs) => {
                format!("({}-{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Mul(ref lhs, ref rhs) => {
                format!("({}*{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Div(ref lhs, ref rhs) => {
                format!("({}/{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Pow(ref lhs, ref rhs) => format!(
                "pow({},{})",
                self.compile_expr(lhs)?,
                self.compile_expr(rhs)?
            ),
            Expr::Mod(ref lhs, ref rhs) => format!("((int){}%(int){})", self.compile_expr(lhs)?, self.compile_expr(rhs)?),
            Expr::Leq(ref lhs, ref rhs) => {
                format!("({}<={})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Geq(ref lhs, ref rhs) => {
                format!("({}>={})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Lt(ref lhs, ref rhs) => {
                format!("({}<{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Gt(ref lhs, ref rhs) => {
                format!("({}>{})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Eq(ref lhs, ref rhs) => {
                format!("({}=={})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
            Expr::Neq(ref lhs, ref rhs) => {
                format!("({}!={})", self.compile_expr(lhs)?, self.compile_expr(rhs)?)
            }
        })
    }
}
