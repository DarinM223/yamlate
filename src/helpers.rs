use ast::{Exp, Op};
use errors::{LexError, YamlError};
use std::collections::HashMap;

lazy_static! {
    static ref OPERATORS: HashMap<String, i32> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("(".to_owned(), -1); // "(" ignores operator precedence
        hash_map.insert(")".to_owned(), 8);

        hash_map.insert("!".to_owned(), 7);

        hash_map.insert("^".to_owned(), 6);

        hash_map.insert("*".to_owned(), 5);
        hash_map.insert("/".to_owned(), 5);
        hash_map.insert("%".to_owned(), 5);

        hash_map.insert("+".to_owned(), 4);
        hash_map.insert("-".to_owned(), 4);

        hash_map.insert("!=".to_owned(), 3);
        hash_map.insert("==".to_owned(), 3);

        hash_map.insert("&&".to_owned(), 2);
        hash_map.insert("||".to_owned(), 1);

        hash_map.insert("=".to_owned(), 0);
        hash_map.insert(":=".to_owned(), 0);

        hash_map
    };
}

pub fn is_keyword(string: &str) -> bool {
    match string {
        "if" => true,
        "then" => true,
        "elif" => true,
        "else" => true,
        "do" => true,
        _ => false,
    }
}

pub fn is_operator(string: &str) -> bool {
    OPERATORS.contains_key(string)
}

pub fn operator_precedence(string: &str) -> i32 {
    match OPERATORS.get(string) {
        Some(result) => *result,
        None => -1,
    }
}

pub fn operator_to_exp(operator: &str, exp1: Exp, exp2: Exp) -> Result<Exp, YamlError> {
    Ok(match operator {
        "==" => Exp::BinaryOp(Op::Equal, Box::new(exp1), Box::new(exp2)),
        "!=" => Exp::BinaryOp(Op::NotEqual, Box::new(exp1), Box::new(exp2)),
        "+" => Exp::BinaryOp(Op::Plus, Box::new(exp1), Box::new(exp2)),
        "-" => Exp::BinaryOp(Op::Minus, Box::new(exp1), Box::new(exp2)),
        "*" => Exp::BinaryOp(Op::Times, Box::new(exp1), Box::new(exp2)),
        "/" => Exp::BinaryOp(Op::Divide, Box::new(exp1), Box::new(exp2)),
        "%" => Exp::BinaryOp(Op::Modulo, Box::new(exp1), Box::new(exp2)),
        "^" => Exp::BinaryOp(Op::Exponent, Box::new(exp1), Box::new(exp2)),
        "&&" => Exp::BinaryOp(Op::And, Box::new(exp1), Box::new(exp2)),
        "||" => Exp::BinaryOp(Op::Or, Box::new(exp1), Box::new(exp2)),
        "=" => {
            if let Exp::Variable(name) = exp1 {
                Exp::Assign(name, Box::new(exp2))
            } else {
                return Err(YamlError::LexError(LexError::NameNotString));
            }
        }
        ":=" => {
            if let Exp::Variable(name) = exp1 {
                Exp::Declare(name, Box::new(exp2))
            } else {
                return Err(YamlError::LexError(LexError::NameNotString));
            }
        }
        _ => return Err(YamlError::LexError(LexError::UnknownOperator)),
    })
}

pub fn is_split_character(ch: char) -> bool {
    match ch {
        ' ' => true,
        '\n' => true,
        '\t' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_keyword() {
        assert_eq!(is_keyword("if"), true);
        assert_eq!(is_keyword("then"), true);
        assert_eq!(is_keyword("elif"), true);

        assert_eq!(is_keyword("hello"), false);
    }

    #[test]
    fn test_is_operator() {
        assert_eq!(is_operator("="), true);
        assert_eq!(is_operator("+"), true);

        assert_eq!(is_operator("~"), false);
    }

    #[test]
    fn test_is_split_character() {
        assert_eq!(is_split_character(' '), true);
        assert_eq!(is_split_character('\n'), true);
        assert_eq!(is_split_character('\t'), true);

        assert_eq!(is_split_character('a'), false);
    }

    #[test]
    fn test_operator_precedence() {
        assert!(operator_precedence("*") > operator_precedence("+"));
        assert!(operator_precedence("+") < operator_precedence("/"));
        assert!(operator_precedence("hello") == -1);
    }
}
