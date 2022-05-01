use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit0, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, success},
    error::ParseError,
    multi::{many0, many0_count, many_m_n},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
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
            terminated(program, ws(tag("end"))),
        ),
        |((ident, exprs), body)| Stmt::For {
            body,
            ident: String::from(ident),
            exprs,
        },
    )(input)
}

fn while_loop(input: &str) -> IResult<&str, Stmt> {
    map(
        pair(
            delimited(ws(tag("while")), expr, ws(tag("do"))),
            terminated(program, ws(tag("end"))),
        ),
        |(mut expr, body)| Stmt::While {
            body,
            expr: shunting_yard(&mut expr),
        },
    )(input)
}

fn declaration(input: &str) -> IResult<&str, Stmt> {
    alt((
        map(
            preceded(ws(tag("local")), separated_pair(ident, ws(tag("=")), expr)),
            |(ident, mut expr)| Stmt::Declaration {
                name: String::from(ident),
                value: shunting_yard(&mut expr),
            },
        ),
        map(preceded(ws(tag("local")), ident), |s| Stmt::Declaration {
            name: String::from(s),
            value: Box::new(Expr::Number(0.0)),
        }),
    ))(input)
}

fn assignment(input: &str) -> IResult<&str, Stmt> {
    map(
        pair(terminated(ident, ws(tag("="))), expr),
        |(ident, mut expr)| Stmt::Assignment {
            name: String::from(ident),
            value: shunting_yard(&mut expr),
        },
    )(input)
}

fn if_block(input: &str) -> IResult<&str, Stmt> {
    map(
        pair(
            delimited(ws(tag("if")), expr, ws(tag("do"))),
            pair(
                program,
                opt(alt((
                    preceded(ws(tag("else")), map(if_block, |e| vec![e])),
                    preceded(ws(tag("else")), program),
                ))),
            ),
        ),
        |(mut expr, (body, branch))| Stmt::IfStatement {
            cond: shunting_yard(&mut expr),
            body,
            branch,
        },
    )(input)
}

fn if_stmt(input: &str) -> IResult<&str, Stmt> {
    terminated(if_block, ws(tag("end")))(input)
}

fn stmt_expr(input: &str) -> IResult<&str, Stmt> {
    map(expr, |mut e| Stmt::Expression(shunting_yard(&mut e)))(input)
}

fn function_def(input: &str) -> IResult<&str, Stmt> {
    alt((
        map(
            separated_pair(
                pair(
                    ident,
                    delimited(
                        ws(tag("(")),
                        opt(pair(ident, many0(preceded(ws(tag(",")), ident)))),
                        ws(tag(")")),
                    ),
                ),
                ws(tag("=")),
                stmt_expr,
            ),
            |((ident, params), expr)| Stmt::FunctionDefinition {
                name: String::from(ident),
                args: if let Some((args0, args)) = params {
                    let mut v = vec![String::from(args0)];
                    v.extend(args.into_iter().map(String::from));
                    v
                } else {
                    vec![]
                },
                body: vec![expr],
            },
        ),
        map(
            pair(
                pair(
                    ident,
                    delimited(
                        ws(tag("(")),
                        opt(pair(ident, many0(preceded(ws(tag(",")), ident)))),
                        ws(tag(")")),
                    ),
                ),
                delimited(ws(tag("do")), program, ws(tag("end"))),
            ),
            |((name, params), body)| Stmt::FunctionDefinition {
                name: String::from(name),
                args: if let Some((args0, args)) = params {
                    let mut v = vec![String::from(args0)];
                    v.extend(args.into_iter().map(String::from));
                    v
                } else {
                    vec![]
                },
                body,
            },
        ),
    ))(input)
}

fn float(input: &str) -> IResult<&str, f64> {
    map_res(
        alt((recognize(tuple((digit0, tag("."), digit1))), digit1)),
        |e: &str| e.parse(),
    )(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    let (i, ident) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    match ident {
        "do" | "end" | "for" | "while" | "if" | "else" | "local" => Err(nom::Err::Error(
            nom::error::ParseError::from_error_kind(i, nom::error::ErrorKind::Tag),
        )),
        _ => Ok((i, ident)),
    }
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
            tag("%"),
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
                "%" => Mod,
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
    alt((
        for_loop,
        while_loop,
        if_stmt,
        function_def,
        assignment,
        declaration,
        stmt_expr,
    ))(input)
}

pub fn program(input: &str) -> IResult<&str, Program> {
    many0(stmt)(input)
}

pub fn parse(input: &str) -> Program {
    let p = program(input);

    match p {
        Ok(v) => v.1,
        Err(_) => {
            eprintln!("failed to parse input");
            std::process::exit(1)
        }
    }
}
