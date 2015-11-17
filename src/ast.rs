#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub enum AST {
    Assign(Box<AST>, Box<AST>),
    Plus(Box<AST>, Box<AST>),
    Minus(Box<AST>, Box<AST>),
    Times(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    Modulo(Box<AST>, Box<AST>),
    Exponent(Box<AST>, Box<AST>),
    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Not(Box<AST>),

    Variable(String),
    Number(String),
    String(String),
    None,
}
