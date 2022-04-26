use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit0, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, success},
    error::ParseError,
    multi::{many0, many0_count, many1, many_m_n},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::shunting_yard::shunting_yard;
use crate::types::*;

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn for_loop(input: &str) -> IResult<&str, Stmt> {
    map(
        pair(
            delimited(
                ws(tag("for")),
                pair(
                    ident,
                    many_m_n(
                        1,
                        3,
                        preceded(ws(tag(",")), map(expr, |mut e| shunting_yard(&mut e))),
                    ),
                ),
                ws(tag("do")),
            ),
            terminated(tag("1"), ws(tag("end"))),
        ),
        |((ident, exprs), body)| Stmt::For {
            body: vec![],
            ident: String::from(ident),
            exprs,
        },
    )(input)
}

fn stmt_expr(input: &str) -> IResult<&str, Stmt> {
    map(expr, |mut e| Stmt::Expression(shunting_yard(&mut e)))(input)
}

fn float(input: &str) -> IResult<&str, f64> {
    println!("{}", input);
    map_res(
        recognize(tuple((digit0, opt(tag(".")), digit1))),
        |e: &str| {
            println!("{}", e);
            e.parse()
        },
    )(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn unary(input: &str) -> IResult<&str, ExprToken> {
    alt((
        map(float, |v| ExprToken::Number(v)),
        map(
            pair(ident, delimited(ws(tag("(")), expr_list, ws(tag(")")))),
            |(i, e)| ExprToken::Call {
                name: String::from(i),
                args: e,
            },
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
            },
        ),
        map(unary, |u| vec![u]),
    ))(input)
}

fn op(input: &str) -> IResult<&str, ExprToken> {
    map(
        alt((
            tag("+"),
            tag("-"),
            tag("*"),
            tag("/"),
            tag("^"),
            tag("<="),
            tag(">="),
            tag("<"),
            tag(">"),
            tag("=="),
            tag("!="),
        )),
        |op| {
            use ExprToken::*;
            match op {
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
                "^" => Pow,
                "<=" => Leq,
                ">=" => Geq,
                "<" => Lt,
                ">" => Gt,
                "==" => Eq,
                "!=" => Neq,
                _ => unreachable!(),
            }
        },
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
        },
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
            },
        ),
        success(vec![]),
    ))(input)
}

fn stmt(input: &str) -> IResult<&str, Stmt> {
    alt((for_loop, stmt_expr))(input)
}

pub fn program(input: &str) -> IResult<&str, Program> {
    many1(stmt)(input)
}
