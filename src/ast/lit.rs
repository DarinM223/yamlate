use errors::EvalError;
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
    pub fn and(&self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (&Bool(b1), Bool(b2)) => Ok(Lit::Bool(b1 && b2)),
            _ => Err(EvalError::new("Invalid types anded")),
        }
    }

    pub fn or(&self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (&Bool(b1), Bool(b2)) => Ok(Lit::Bool(b1 || b2)),
            _ => Err(EvalError::new("Invalid types ored")),
        }
    }

    pub fn exp(&self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (&Number(n1), Number(n2)) => Ok(Lit::Number((n1 as f64).powi(n2) as i32)),
            (&Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64).powf(d))),
            (&Decimal(d), Number(n)) => Ok(Lit::Decimal(d.powi(n))),
            _ => Err(EvalError::new("Invalid types exped")),
        }
    }
}

impl Add<Lit> for Lit {
    type Output = Result<Lit, EvalError>;

    fn add(self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 + n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal(d + (n as f64))),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d + (n as f64))),
            (Str(s1), Str(s2)) => Ok(Lit::Str(s1 + &s2)),
            _ => Err(EvalError::new("Invalid types added")),
        }
    }
}

impl Sub<Lit> for Lit {
    type Output = Result<Lit, EvalError>;

    fn sub(self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 - n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) - d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d - (n as f64))),
            _ => Err(EvalError::new("Invalid types subtracted")),
        }
    }
}

impl Mul<Lit> for Lit {
    type Output = Result<Lit, EvalError>;

    fn mul(self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 * n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) * d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d * (n as f64))),
            _ => Err(EvalError::new("Invalid types multiplied")),
        }
    }
}

impl Div<Lit> for Lit {
    type Output = Result<Lit, EvalError>;

    fn div(self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 / n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) / d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d / (n as f64))),
            _ => Err(EvalError::new("Invalid types divided")),
        }
    }
}

impl Rem<Lit> for Lit {
    type Output = Result<Lit, EvalError>;

    fn rem(self, other: Lit) -> Result<Lit, EvalError> {
        match (self, other) {
            (Number(n1), Number(n2)) => Ok(Lit::Number(n1 % n2)),
            (Number(n), Decimal(d)) => Ok(Lit::Decimal((n as f64) % d)),
            (Decimal(d), Number(n)) => Ok(Lit::Decimal(d % (n as f64))),
            _ => Err(EvalError::new("Invalid types divided")),
        }
    }
}

impl Not for Lit {
    type Output = Result<Lit, EvalError>;

    fn not(self) -> Result<Lit, EvalError> {
        match self {
            Bool(b) => Ok(Lit::Bool(!b)),
            _ => Err(EvalError::new("Invalid type to not")),
        }
    }
}
