use crate::{Expr, ExprToken};

fn precedence(op: &ExprToken) -> i16 {
    use ExprToken::*;
    match op {
        Eq => 2,
        Neq => 2,

        Leq => 3,
        Geq => 3,
        Lt => 3,
        Gt => 3,

        Add => 4,
        Sub => 4,

        Mul => 5,
        Div => 5,

        Pow => 6,

        LParen => 0,
        RParen => -1,
        _ => unreachable!(),
    }
}

fn is_op(e: &ExprToken) -> bool {
    use ExprToken::*;
    match e {
        Add | Sub | Mul | Div | Pow | Leq | Geq | Lt | Gt | Eq | Neq | LParen | RParen => true,
        _ => false,
    }
}

pub fn shunting_yard(tokens: &mut Vec<ExprToken>) -> Box<Expr> {
    let mut rpn = Vec::with_capacity(tokens.len());
    let mut op_stack: Vec<(i16, &mut ExprToken)> = Vec::new();

    for token in tokens.into_iter() {
        if is_op(token) {
            let prec = precedence(token);

            while !op_stack.is_empty() {
                if *token == ExprToken::LParen {
                    break;
                } else if prec < op_stack.last().unwrap().0 {
                    let op = op_stack.pop().unwrap();

                    if *op.1 == ExprToken::LParen && *token == ExprToken::RParen {
                        break;
                    } else {
                        rpn.push(op.1)
                    }
                }
                /* TODO equal precedence left associative popping */
                else {
                    break;
                }
            }

            if *token != ExprToken::RParen {
                op_stack.push((prec, token));
            }
        } else {
            rpn.push(token);
        }
    }

    while !op_stack.is_empty() {
        let op = op_stack.pop().unwrap();

        if *op.1 != ExprToken::LParen {
            rpn.push(op.1)
        }
    }

    let mut expr_trees = vec![];

    for token in rpn.into_iter() {
        use ExprToken::*;
        if is_op(token) {
            let rhs = expr_trees.pop().unwrap();
            let lhs = expr_trees.pop().unwrap();
            let new_expr = match token {
                Add => Expr::Add(lhs, rhs),
                Sub => Expr::Sub(lhs, rhs),
                Mul => Expr::Mul(lhs, rhs),
                Div => Expr::Div(lhs, rhs),
                Pow => Expr::Pow(lhs, rhs),
                Leq => Expr::Leq(lhs, rhs),
                Geq => Expr::Geq(lhs, rhs),
                Lt => Expr::Lt(lhs, rhs),
                Gt => Expr::Gt(lhs, rhs),
                Eq => Expr::Eq(lhs, rhs),
                Neq => Expr::Neq(lhs, rhs),
                _ => unreachable!(),
            };
            expr_trees.push(Box::new(new_expr));
        } else {
            expr_trees.push(Box::new(match token {
                Number(v) => Expr::Number(*v),
                Ident(s) => Expr::Ident(s.clone()),
                Call { name, args } => Expr::Call {
                    name: name.clone(),
                    args: args.into_iter().map(shunting_yard).collect(),
                },
                _ => unreachable!(),
            }));
        }
    }

    expr_trees.swap_remove(0)
}
