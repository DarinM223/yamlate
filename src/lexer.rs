use ast::AST;
use helpers::is_operator;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
enum WordState {
    Variable,
    Number,
    Decimal,
    String,
    Operator,
    None,
}

fn split_string_letter(ch: char,
                       _variable_array: &mut VecDeque<AST>,
                       operator_array: &mut VecDeque<String>,
                       curr_state: &WordState,
                       curr_str: String)
                       -> Result<(String, WordState), String> {
    match curr_state {
        &ref state @ WordState::Variable | &ref state @ WordState::String =>
            Ok((curr_str + ch.to_string().as_str(), state.clone())),
        &WordState::Number | &WordState::Decimal =>
            Err("Number cannot have a letter after it".to_string()),
        &WordState::Operator => {
            operator_array.push_front(curr_str);
            Ok((ch.to_string(), WordState::Variable))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
    }
}

fn split_string_digit(ch: char,
                      _variable_array: &mut VecDeque<AST>,
                      operator_array: &mut VecDeque<String>,
                      curr_state: &WordState,
                      curr_str: String)
                      -> Result<(String, WordState), String> {
    match curr_state {
        &ref state @ WordState::Variable |
        &ref state @ WordState::Number |
        &ref state @ WordState::Decimal |
        &ref state @ WordState::String => Ok((curr_str + ch.to_string().as_str(), state.clone())),
        &WordState::Operator => {
            operator_array.push_front(curr_str);
            Ok((ch.to_string(), WordState::Number))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
    }
}

fn split_string_operator(ch: char,
                         variable_array: &mut VecDeque<AST>,
                         operator_array: &mut VecDeque<String>,
                         curr_state: &WordState,
                         curr_str: String)
                         -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::Variable => {
            variable_array.push_front(AST::Variable(curr_str));
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::Number => {
            variable_array.push_front(AST::Number(curr_str.as_str().parse().unwrap()));
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::Decimal => {
            variable_array.push_front(AST::Decimal(curr_str.as_str().parse().unwrap()));
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Operator => {
            let new_str;
            if is_operator((curr_str.clone() + ch.to_string().as_str()).as_str()) {
                new_str = curr_str + ch.to_string().as_str();
            } else {
                operator_array.push_front(curr_str);
                new_str = ch.to_string();
            }

            Ok((new_str, WordState::Operator))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Operator)),
    }
}

fn split_string_quote(_ch: char,
                      variable_array: &mut VecDeque<AST>,
                      operator_array: &mut VecDeque<String>,
                      curr_state: &WordState,
                      curr_str: String)
                      -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::String => {
            variable_array.push_front(AST::String(curr_str));
            Ok((String::new(), WordState::None))
        }
        &WordState::Number | &WordState::Decimal | &WordState::Variable =>
            Err("Cannot create a string after invalid type".to_string()),
        &WordState::Operator => {
            operator_array.push_front(curr_str);
            Ok((String::new(), WordState::String))
        }
        &WordState::None => Ok((curr_str, WordState::String)),
    }
}

fn split_string_dot(ch: char,
                    _variable_array: &mut VecDeque<AST>,
                    _operator_array: &mut VecDeque<String>,
                    curr_state: &WordState,
                    curr_str: String)
                    -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Number => Ok((curr_str + ch.to_string().as_str(), WordState::Decimal)),
        &WordState::Operator | &WordState::Decimal | &WordState::Variable =>
            Err("Cannot have a dot after".to_string()),
        &WordState::None => Err("Cannot start with dot".to_string()),
    }
}

/// Parses a word split from split_string and detects operators 
/// Returns a deque of variables/constants and a deque of operators
/// TODO: clean up this mess
pub fn parse_string(s: &str) -> Result<(VecDeque<AST>, VecDeque<String>), String> {
    let mut variable_array: VecDeque<AST> = VecDeque::new();
    let mut operator_array = VecDeque::new();

    let mut curr_state = WordState::None;
    let mut curr_str = String::new();

    for ch in s.to_string().chars() {
        let split_fn: fn(char, &mut VecDeque<AST>, &mut VecDeque<String>, &WordState, String)
    -> Result<(String, WordState), String>;

        if ch.is_alphabetic() {
            split_fn = split_string_letter;
        } else if ch.is_digit(10) {
            split_fn = split_string_digit;
        } else if ch == ' ' || ch == '\t' {
            // ignore spaces and tabs except for inside a string
            if curr_state == WordState::String {
                curr_str = curr_str + ch.to_string().as_str();
            }
            continue;
        } else if ch == '\"' {
            split_fn = split_string_quote;
        } else if ch == '.' {
            split_fn = split_string_dot;
        } else {
            split_fn = split_string_operator;
        }

        match split_fn(ch,
                       &mut variable_array,
                       &mut operator_array,
                       &mut curr_state,
                       curr_str) {
            Ok((new_str, new_state)) => {
                curr_str = new_str;
                curr_state = new_state;
            }
            Err(e) => return Err(e),
        }
    }

    // after string is finished, add the currently built string into the result
    if curr_str.len() > 0 {
        match curr_state {
            WordState::Variable => variable_array.push_front(AST::Variable(curr_str)),
            WordState::Number =>
                variable_array.push_front(AST::Number(curr_str.as_str().parse().unwrap())),
            WordState::Decimal =>
                variable_array.push_front(AST::Decimal(curr_str.as_str().parse().unwrap())),
            WordState::String => variable_array.push_front(AST::String(curr_str)),
            WordState::Operator => operator_array.push_front(curr_str),
            WordState::None => {}
        }
    }

    return Ok((variable_array, operator_array));
}

#[test]
fn test_no_paren() {
    let s = "a+2-b+3";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_string()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_string()),
                                                      AST::Number(3)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["+", "-", "+"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_string())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_paren() {
    let s = "(a+(2-b)+(3*5))";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_string()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_string()),
                                                      AST::Number(3),
                                                      AST::Number(5)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "+", "(", "-", ")", "+", "(", "*",
                                                         ")", ")"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_string())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_equals() {
    let s = "(a==(2-b)+(3!=5))";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_string()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_string()),
                                                      AST::Number(3),
                                                      AST::Number(5)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "==", "(", "-", ")", "+", "(", "!=",
                                                         ")", ")"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_string())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_spaces() {
    let s = "( a + 2 - \t b \t^ 2 ) == 5";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_string()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_string()),
                                                      AST::Number(2),
                                                      AST::Number(5)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "+", "-", "^", ")", "=="]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_string())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_strings() {
    let s = "( \"Hello world1234 + \" + \"bye123\" )";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::String("Hello world1234 + "
                                                                      .to_string()),
                                                      AST::String("bye123".to_string())]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "+", ")"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_string())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_float() {
    let s = "1.23 - 3.14 + 123.45678";
    match parse_string(s) {
        Ok((variables, _)) => {
            let variable_result: VecDeque<AST> = vec![AST::Decimal(1.23),
                                                      AST::Decimal(3.14),
                                                      AST::Decimal(123.45678)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);
        }
        Err(e) => println!("{:?}", e),
    }
}
