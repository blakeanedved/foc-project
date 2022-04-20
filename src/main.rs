extern crate nom;

mod shunting_yard;
mod compiler;
mod types;

use shunting_yard::shunting_yard;

use nom::{
  IResult,
  error::ParseError,
  branch::alt,
  bytes::complete::tag,
  combinator::{map, recognize, success},
  character::complete::{multispace0, i32, alpha1, alphanumeric1},
  sequence::{delimited, pair, preceded}, multi::{many0, many0_count}};

#[derive(Debug, Clone, PartialEq)]
pub enum ExprToken {
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
    Number(i32),
    Ident(String),
    Call { name: String, args: Vec<Vec<ExprToken>> }
}

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Number(i32),
    Ident(String),
    Call { name: String, args: Vec<Box<Expr>> }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}

fn ident(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

fn unary(input: &str) -> IResult<&str, ExprToken> {
    alt((
        map(i32, |v| ExprToken::Number(v)),
        map(
            pair(ident, delimited(ws(tag("(")), expr_list, ws(tag(")")))),
            |(i, e)| ExprToken::Call{ name: String::from(i), args: e }
        ),
        map(ident, |s| ExprToken::Ident(String::from(s))),
    ))(input)
}

fn term(input: &str) -> IResult<&str, Vec<ExprToken>> {
    alt((
        map(
            delimited(ws(tag("(")), expr, ws(tag(")"))),
            |mut e: Vec<ExprToken>| {
                e.insert(0, ExprToken::LParen);
                e.push(ExprToken::RParen);
                e
            }
        ),
        map(unary, |u| vec![u])
    ))(input)
}

fn op(input: &str) -> IResult<&str, ExprToken> {
    map(
        alt((tag("+"), tag("-"), tag("*"), tag("/"))),
        |op| {
            use ExprToken::*;
            match op {
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
                _ => unreachable!()
            }
        }
    )(input)
}

fn expr(input: &str) -> IResult<&str, Vec<ExprToken>> {
    map(
        pair(ws(term), many0(pair(ws(op), ws(term)))),
        |(mut e, l)| {
            for (op, e2) in l {
                e.push(op);
                e.extend(e2.into_iter());
            }
            e
        }
    )(input)
}

fn expr_list(input: &str) -> IResult<&str, Vec<Vec<ExprToken>>> {
    alt((
        map(
            pair(expr, many0(preceded(ws(tag(",")), expr))),
            |(e, el)| {
                let mut v = vec![e];
                for e in el {
                    v.push(e);
                }
                v
            }
        ),
        success(vec![])
    ))(input)
}

fn main() {
  println!("{:?}", shunting_yard(&mut expr("f(x*2, y+(5/3))").unwrap().1));
  println!("{:?}", shunting_yard(&mut expr("5 * f ( x )").unwrap().1));
}
