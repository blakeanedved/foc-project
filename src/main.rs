extern crate nom;

mod compiler;
mod parser;
mod shunting_yard;
mod types;
mod utils;

use compiler::Compiler;
use parser::parse;
use types::*;

use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
#[clap(author = "Blake Nedved and Aaron Ingalls", version = "0.1")]
pub struct Args {
    filename: String,

    #[clap(short = 's', long)]
    intermediates: bool,

    #[clap(
        short,
        long,
        help = "The file that the compiled program will output to"
    )]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let code = read_to_string(&args.filename)?;

    let program = parse(&code);

    Compiler::compile(&program, args)?;

    Ok(())
}
