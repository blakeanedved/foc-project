extern crate nom;

mod compiler;
mod parser;
mod shunting_yard;
mod types;

use parser::*;
use shunting_yard::shunting_yard;
use types::*;

fn main() -> anyhow::Result<()> {
    // println!(
    //     "{:?}",
    //     shunting_yard(&mut parser::expr("f(x*2, y+(5/3))")?.1)
    // );
    // println!("{:?}", shunting_yard(&mut expr("4 ^ 3 ^ 2")?.1));
    println!("{:?}", parser::program("3")?);
    // println!("{:?}", parser::program("for i,n do 1 end")?);

    // compiler::compile(vec![Stmt::Expression(shunting_yard(&mut expr("1+2-3*4/5^6")?.1))])?;

    Ok(())
}
