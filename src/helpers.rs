#[macro_use]

use std::collections::HashMap;
use std::sync::{Once, ONCE_INIT};
use regex::Regex;

lazy_static! {
    static ref OPERATORS: HashMap<String, i32> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("=".to_string(), 0);
        hash_map.insert("==".to_string(), 1);
        hash_map.insert("!=".to_string(), 1);
        hash_map.insert("+".to_string(), 2);
        hash_map.insert("*".to_string(), 3);
        hash_map.insert("/".to_string(), 3);
        hash_map.insert("%".to_string(), 3);
        hash_map.insert("^".to_string(), 4);
        hash_map.insert("(".to_string(), 5);
        hash_map.insert(")".to_string(), 5);

        hash_map
    };
}

fn is_keyword(string: &str) -> bool {
    match string {
        "if" => true,
        "then" => true,
        "elif" => true,
        "else" => true,
        "do" => true,
        _ => false,
    }
}

fn is_operator(string: &str) -> bool {
    OPERATORS.contains_key(string)
}

fn operator_precedence(string: &str) -> i32 {
    match OPERATORS.get(string) {
        Some(result) => *result,
        None => -1,
    }
}

fn is_split_character(ch: char) -> bool {
    match ch {
        ' ' => true,
        '\n' => true,
        '\t' => true,
        _ => false,
    }
}

/// Removes whitespace, tabs, and newlines and splits the string based on them
pub fn split_string(s: &str) -> Vec<String> {
    let re = Regex::new(r"[ \t\n]+").unwrap();
    re.split(s).map(|s| s.to_string()).collect::<Vec<_>>()
}

enum WordState {
    Variable,
    Number,
    String,
    Operator,
    None,
}

/// Parses a word split from split_string and detects operators 
/// Returns an array of variables/constants and an array of operators
/// TODO: clean up this mess
pub fn split_word(s: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let mut variable_array = Vec::new();
    let mut operator_array = Vec::new();

    let mut curr_state = WordState::None;
    let mut curr_str = String::new();

    for ch in s.to_string().chars() {
        if ch.is_alphabetic() {
            match curr_state {
                WordState::Variable | WordState::String => curr_str = curr_str + ch.to_string().as_str(),
                WordState::Number => return Err("Number cannot have a letter after it".to_string()),
                WordState::Operator => {
                    operator_array.push(curr_str);
                    curr_str = String::new() + ch.to_string().as_str();
                    curr_state = WordState::Variable;
                },
                WordState::None => {
                    curr_str = curr_str + ch.to_string().as_str();
                    curr_state = WordState::Variable;
                },
            }
        } else if ch.is_digit(10) {
            match curr_state {
                WordState::Variable | WordState::Number | WordState::String => 
                    curr_str = curr_str + ch.to_string().as_str(),
                WordState::Operator => {
                    operator_array.push(curr_str);
                    curr_str = String::new() + ch.to_string().as_str();
                    curr_state = WordState::Number;
                },
                WordState::None => {
                    curr_str = curr_str + ch.to_string().as_str();
                    curr_state = WordState::Number;
                },
            }
        } else {
            match curr_state {
                WordState::Variable | WordState::Number | WordState::String => {
                    variable_array.push(curr_str);
                    curr_str = String::new() + ch.to_string().as_str();
                    curr_state = WordState::Operator;
                },
                WordState::Operator => {
                    if is_operator((curr_str.clone() + ch.to_string().as_str()).as_str()) {
                        curr_str = curr_str + ch.to_string().as_str();
                    } else {
                        operator_array.push(curr_str);
                        curr_str = String::new() + ch.to_string().as_str();
                        curr_state = WordState::Operator;
                    }
                },
                WordState::None => {
                    curr_str = curr_str + ch.to_string().as_str();
                    curr_state = WordState::Operator;
                }
            }
        }
    }

    if curr_str.len() > 0 {
        match curr_state {
            WordState::Variable | WordState::Number | WordState::String => 
                variable_array.push(curr_str),
            WordState::Operator => operator_array.push(curr_str),
            WordState::None => {},
        }
    }

    return Ok((variable_array, operator_array));
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

#[test]
fn test_split_word_no_paren() {
    let s = "a+2-b+3";
    match split_word(s) {
        Ok((variables, operators)) => {
            let variable_result = vec!["a", "2", "b", "3"];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i].to_string());
            }
            let operator_result = vec!["+", "-", "+"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i].to_string());
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_split_word_paren() {
    let s = "(a+(2-b)+(3*5))";
    match split_word(s) {
        Ok((variables, operators)) => {
            let variable_result = vec!["a", "2", "b", "3", "5"];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i].to_string());
            }
            let operator_result = vec!["(", "+", "(", "-", ")", "+", "(", "*", ")", ")"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i].to_string());
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_split_word_equals() {
    let s = "(a==(2-b)+(3!=5))";
    match split_word(s) {
        Ok((variables, operators)) => {
            let variable_result = vec!["a", "2", "b", "3", "5"];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i].to_string());
            }
            let operator_result = vec!["(", "==", "(", "-", ")", "+", "(", "!=", ")", ")"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i].to_string());
            }
        },
        Err(e) => println!("{:?}", e),
    }
}