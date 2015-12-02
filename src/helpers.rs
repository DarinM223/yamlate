use std::collections::HashMap;
use ast::AST;

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

pub fn ast_to_operator(ast: &AST) -> String {
    match *ast {
        AST::Declare(_, _) => ":=",
        AST::Assign(_, _) => "=",
        AST::Equal(_, _) => "==",
        AST::NotEqual(_, _) => "!=",
        AST::Plus(_, _) => "+",
        AST::Minus(_, _) => "-",
        AST::Times(_, _) => "*",
        AST::Divide(_, _) => "/",
        AST::Modulo(_, _) => "%",
        AST::Exponent(_, _) => "^",
        AST::And(_, _) => "&&",
        AST::Or(_, _) => "||", 
        AST::Not(_) => "!",
        _ => "",
    }
    .to_owned()
}

pub fn operator_to_ast(operator: &str, ast1: AST, ast2: AST) -> AST {
    match operator {
        "=" => AST::Assign(box ast1, box ast2),
        ":=" => AST::Declare(box ast1, box ast2),
        "==" => AST::Equal(box ast1, box ast2),
        "!=" => AST::NotEqual(box ast1, box ast2),
        "+" => AST::Plus(box ast1, box ast2),
        "-" => AST::Minus(box ast1, box ast2),
        "*" => AST::Times(box ast1, box ast2),
        "/" => AST::Divide(box ast1, box ast2),
        "%" => AST::Modulo(box ast1, box ast2),
        "^" => AST::Exponent(box ast1, box ast2),
        "&&" => AST::And(box ast1, box ast2),
        "||" => AST::Or(box ast1, box ast2),
        _ => AST::None,
    }
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
