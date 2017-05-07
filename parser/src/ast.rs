use std::fmt::{Debug, Formatter, Error};

#[derive(Clone, Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Ref(Ident),
    Do(Vec<Vec<Expr>>, Option<Vec<Statement>>),
    Parens(Vec<Expr>),
    Error,
    Dummy,
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Data(Ident, Vec<Ident>, Vec<Ident>),
    Dummy,
    Class,
    Instance,
    Import,
    Newtype,
    Pipelist,
    GuardAssign,
    Assign(Ident, Vec<Ident>, Vec<Expr>),
    Typedef(Ident),
    Prototype(Ident, Vec<Ty>),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<Statement>,
}

pub type TySpan = Vec<Ty>;

#[derive(Clone, Debug)]
pub enum Ty {
    Where(Box<Ty>, Box<Ty>),
    Pair(TySpan, Box<Ty>),
    Not(Box<Ty>),
    Ref(Ident),
    EmptyParen,
    Parens(Vec<TySpan>),
    Brackets(Vec<TySpan>),
    Span(TySpan),
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);