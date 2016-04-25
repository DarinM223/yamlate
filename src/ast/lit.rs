use ast::Op;
use errors::{EvalError, YamlError};
use self::Lit::*;
use std::ops::{Add, Mul, Sub, Div, Rem, Not};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Lit {
    Number(i32),
    Bool(bool),
    Decimal(f64),
    Str(String),
    Nil,
}

impl Lit {
    pub fn and(&self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (&Bool(b1), Bool(b2)) => Ok(Lit::Bool(b1 && b2)),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::And, a.clone(), b.clone())))
            }
        }
    }

    pub fn or(&self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (&Bool(b1), Bool(b2)) => Ok(Lit::Bool(b1 || b2)),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Or, a.clone(), b.clone())))
            }
        }
    }

    pub fn exp(&self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (&Number(n1), Number(n2)) => Ok(Lit::Number((n1 as f64).powi(n2) as i32)),
            (&Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64).powf(d))),
            (&Decimal(d), Number(n)) => Ok(Lit::Decimal(d.powi(n))),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Exponent,
                                                                 a.clone(),
                                                                 b.clone())))
            }
        }
    }
}

impl Add<Lit> for Lit {
    type Output = Result<Lit, YamlError>;

    fn add(self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 + n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal(d + (n as f64))),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d + (n as f64))),
            (Str(s1), Str(s2)) => Ok(Lit::Str(s1 + &s2)),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Plus, a.clone(), b.clone())))
            }
        }
    }
}

impl Sub<Lit> for Lit {
    type Output = Result<Lit, YamlError>;

    fn sub(self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 - n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) - d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d - (n as f64))),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Minus, a.clone(), b.clone())))
            }
        }
    }
}

impl Mul<Lit> for Lit {
    type Output = Result<Lit, YamlError>;

    fn mul(self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 * n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) * d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d * (n as f64))),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Times, a.clone(), b.clone())))
            }
        }
    }
}

impl Div<Lit> for Lit {
    type Output = Result<Lit, YamlError>;

    fn div(self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 / n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) / d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d / (n as f64))),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Divide, a.clone(), b.clone())))
            }
        }
    }
}

impl Rem<Lit> for Lit {
    type Output = Result<Lit, YamlError>;

    fn rem(self, other: Lit) -> Result<Lit, YamlError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 % n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) % d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d % (n as f64))),
            (a, b) => {
                Err(YamlError::EvalError(EvalError::InvalidBinOp(Op::Modulo, a.clone(), b.clone())))
            }
        }
    }
}

impl Not for Lit {
    type Output = Result<Lit, YamlError>;

    fn not(self) -> Result<Lit, YamlError> {
        match self {
            Bool(b) => Ok(Lit::Bool(!b)),
            other => Err(YamlError::EvalError(EvalError::InvalidUnOp(Op::Not, other.clone()))),
        }
    }
}
