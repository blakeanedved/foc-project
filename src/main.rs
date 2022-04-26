extern crate nom;

mod compiler;
mod parser;
mod shunting_yard;
mod types;
mod utils;

use compiler::{Compiler, CompilerOptions};
use parser::*;
use shunting_yard::shunting_yard;
use types::*;

use std::path::Path;

fn main() -> anyhow::Result<()> {
    // println!(
    //     "{:?}",
    //     shunting_yard(&mut parser::expr("f(x*2, y+(5/3))")?.1)
    // );
    // println!("{:?}", shunting_yard(&mut expr("4 ^ 3 ^ 2")?.1));
    println!("{:?}", parser::float("1.0")?);
    // println!("{:?}", parser::program("for i,n do 1 end")?);

    let filename = String::from("file.cx");

    Compiler::compile(
        &vec![Stmt::For {
            body: vec![Stmt::Expression(Box::new(Expr::Ident(String::from("i"))))],
            ident: String::from("i"),
            exprs: vec![
                Box::new(Expr::Number(0.0)),
                Box::new(Expr::Number(10.0)),
                Box::new(Expr::Number(1.0)),
            ],
        }],
        CompilerOptions::default(),
        Path::new(&filename)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    )?;

    Ok(())
}
