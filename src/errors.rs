use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LexError {
    desc: String,
}

impl LexError {
    pub fn new(desc: &str) -> LexError {
        LexError { desc: desc.to_owned() }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexer error: {}", self.desc)
    }
}

impl Error for LexError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<EvalError> for LexError {
    fn from(e: EvalError) -> LexError {
        LexError::new(&e.desc[..])
    }
}

#[derive(Debug, PartialEq)]
pub struct EvalError {
    desc: String,
}

impl EvalError {
    pub fn new(desc: &str) -> EvalError {
        EvalError { desc: desc.to_owned() }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluator error: {}", self.desc)
    }
}

impl Error for EvalError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<LexError> for EvalError {
    fn from(e: LexError) -> EvalError {
        EvalError::new(&e.desc[..])
    }
}
