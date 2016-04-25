use ast::{Exp, Lit, Op};

#[derive(Debug, PartialEq)]
pub enum YamlError {
    LexError(LexError),
    EvalError(EvalError),
}

impl YamlError {
    fn description(&self) -> String {
        match *self {
            YamlError::LexError(ref err) => err.description().to_owned(),
            YamlError::EvalError(ref err) => err.description(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    /// When the variable name in an assign or declare is
    /// not a string (+ := 2) (12 := 2)
    NameNotString,
    /// When there is a letter after a number
    /// (12Hello, 5l)
    LetterAfterNumber,
    /// When the operator is not valid ("~", "#")
    UnknownOperator,
    /// When the current state cannot become an expression
    ResultNotLiteral,
    /// When a dot is appended in an invalid state
    /// (1 +. 2, 1.00.2, hel.lo := 2)
    InvalidDotAppend,
    /// When a quote is appended in an invalid state
    /// (12"hello", +"hello", 12.0", name")
    InvalidQuoteAppend,
    /// When an operator cannot be retrieved from the stack
    OperatorStackError,
    /// When a variable cannot be retrieved from the stack
    VariableStackError,
    /// When the lexer cannot completely parse the expressions
    Incomplete,
    /// When the parenthesis do not match ("(1 + 2")
    ParenthesisNotMatch,
}

impl LexError {
    pub fn description(&self) -> &str {
        match *self {
            LexError::NameNotString => "Variable name to assign or declare is not a string",
            LexError::LetterAfterNumber => "Number cannot have a letter after it",
            LexError::UnknownOperator => "Unknown operator",
            LexError::ResultNotLiteral => "Lexed value needs to be a literal",
            LexError::InvalidDotAppend => "Cannot append dot to state",
            LexError::InvalidQuoteAppend => "Cannot append quote to state",
            LexError::OperatorStackError => "Operator cannot be retrieved from the stack",
            LexError::VariableStackError => "Variable cannot be retrieved from the stack",
            LexError::ParenthesisNotMatch => "Parenthesis do not match",
            LexError::Incomplete => "Lexer cannot completely parse expression",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    /// When the type to apply a unary operator is invalid
    /// (!"hello")
    InvalidUnOp(Op, Lit),
    /// When the type to apply a binary operator is invalid
    /// ("hello" - "world")
    InvalidBinOp(Op, Lit, Lit),
    /// When an unary operator's subexpression cannot be reduced to a literal
    CannotReduceUnOp(Op, Box<Exp>),
    /// When a binary operator's subexpressions cannot be reduced to a literal
    CannotReduceBinOp(Op, Box<Exp>, Box<Exp>),
    /// When the expression for an assign cannot be reduced to a literal
    CannotReduceAssign(Box<Exp>),
    /// When the expression for a declare cannot be reduced to a literal
    CannotReduceDeclare(Box<Exp>),
    /// When a variable is not in an environment
    VarNotInEnv(String),
    /// When an operator is not a valid unary operator
    NotUnOp(Op),
    /// When an operator is not a valid binary operator
    NotBinOp(Op),
}

impl EvalError {
    pub fn description(&self) -> String {
        match *self {
            EvalError::InvalidUnOp(op, ref lit) => {
                format!("Invalid type ({:?}) applied to operator {:?}",
                        lit.clone(),
                        op)
            }
            EvalError::InvalidBinOp(op, ref lit1, ref lit2) => {
                format!("Invalid types ({:?}, {:?}) applied to operator {:?}",
                        lit1.clone(),
                        lit2.clone(),
                        op)
            }
            EvalError::CannotReduceUnOp(op, ref exp) => {
                format!("Subexpression ({:?}) cannot be reduced to a value for unary operator {:?}",
                        exp.clone(),
                        op)
            }
            EvalError::CannotReduceBinOp(op, ref exp1, ref exp2) => {
                format!("Subexpressions ({:?}, {:?}) cannot be reduced to a value for binary \
                         operator {:?}",
                        exp1.clone(),
                        exp2.clone(),
                        op)
            }
            EvalError::CannotReduceAssign(ref exp) => {
                format!("Subexpression ({:?}) cannot be reduced to a value for assigning",
                        exp.clone())
            }
            EvalError::CannotReduceDeclare(ref exp) => {
                format!("Subexpression ({:?}) cannot be reduced to a value for declaring",
                        exp.clone())
            }
            EvalError::VarNotInEnv(ref name) => {
                format!("Variable {:?} not in environment", name.clone())
            }
            EvalError::NotUnOp(op) => format!("{:?} is not a unary operator", op),
            EvalError::NotBinOp(op) => format!("{:?} is not a binary operator", op),
        }
    }
}
