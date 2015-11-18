use std::collections::HashMap;
use std::sync::{Once, ONCE_INIT};
use ast::AST;

lazy_static! {
    static ref OPERATORS: HashMap<String, i32> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("(".to_string(), -1); // "(" ignores operator precedence
        hash_map.insert(")".to_string(), 8); 

        hash_map.insert("!".to_string(), 7);

        hash_map.insert("^".to_string(), 6);

        hash_map.insert("*".to_string(), 5);
        hash_map.insert("/".to_string(), 5);
        hash_map.insert("%".to_string(), 5);

        hash_map.insert("+".to_string(), 4);
        hash_map.insert("-".to_string(), 4);

        hash_map.insert("!=".to_string(), 3);
        hash_map.insert("==".to_string(), 3);

        hash_map.insert("&&".to_string(), 2);
        hash_map.insert("||".to_string(), 1);

        hash_map.insert("=".to_string(), 0);

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

pub fn ast_to_operator(ast: &AST) -> String {
    match ast {
        &AST::Assign(_, _) => "=",
        &AST::Plus(_, _) => "+",
        &AST::Minus(_, _) => "-",
        &AST::Times(_, _) => "*",
        &AST::Divide(_, _) => "/",
        &AST::Modulo(_, _) => "%",
        &AST::Exponent(_, _) => "^",
        &AST::And(_, _) => "&&",
        &AST::Or(_, _) => "||", 
        &AST::Not(_) => "!",
        _ => "",
    }
    .to_string()
}

pub fn is_split_character(ch: char) -> bool {
    match ch {
        ' ' => true,
        '\n' => true,
        '\t' => true,
        _ => false,
    }
}

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
