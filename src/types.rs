#[derive(Debug, Clone, PartialEq)]
pub enum ExprToken {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Leq,
    Geq,
    Lt,
    Gt,
    Eq,
    Neq,
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
    Pow(Box<Expr>, Box<Expr>),
    Leq(Box<Expr>, Box<Expr>),
    Geq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Number(i32),
    Ident(String),
    Call { name: String, args: Vec<Box<Expr>> }
}

pub type Program = Vec<Stmt>;

pub enum Stmt {
    
}
