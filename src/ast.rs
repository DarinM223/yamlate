#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum AST {
    Declare(Box<AST>, Box<AST>),
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
    Equal(Box<AST>, Box<AST>),
    NotEqual(Box<AST>, Box<AST>),

    Variable(String),
    Number(i32),
    Decimal(f64),
    String(String),
    None,
}
