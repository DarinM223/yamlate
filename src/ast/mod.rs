pub mod exp;
pub mod lit;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Exponent,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
}

pub use ast::exp::Exp;
pub use ast::lit::Lit;
