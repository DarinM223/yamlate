pub mod exp;
pub mod lit;

#[derive(Debug, PartialEq, Clone, Copy)]
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

pub use crate::ast::exp::Exp;
pub use crate::ast::lit::Lit;
