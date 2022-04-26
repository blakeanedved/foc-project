extern crate nom;

mod compiler;
mod parser;
mod shunting_yard;
mod types;

use compiler::Compiler;
use parser::*;
use shunting_yard::shunting_yard;
use types::*;

fn main() -> anyhow::Result<()> {
    // println!(
    //     "{:?}",
    //     shunting_yard(&mut parser::expr("f(x*2, y+(5/3))")?.1)
    // );
    // println!("{:?}", shunting_yard(&mut expr("4 ^ 3 ^ 2")?.1));
    println!("{:?}", parser::float("1.0")?);
    // println!("{:?}", parser::program("for i,n do 1 end")?);

    // let mut c = Compiler::new();
    // let p = c.compile(vec![Stmt::FunctionDefinition {
    //     name: String::from("foo"),
    //     args: vec![String::from("x")],
    //     body: vec![Stmt::FunctionDefinition {
    //         name: String::from("foo"),
    //         args: vec![String::from("x")],
    //         body: vec![Stmt::Expression(Box::new(Expr::Number(1.0)))],
    //     }],
    // }])?;
    // println!("==============================\n{}", p);

    Ok(())
}
