use ast::AST;
use helpers::is_operator;
use std::collections::VecDeque;
use errors::LexError;

#[derive(Clone, PartialEq)]
enum WordState {
    Variable,
    Number,
    Decimal,
    String,
    Operator,
    None,
}

#[derive(Clone, PartialEq)]
enum StringBuildType {
    Letter,
    Digit,
    Operator,
    Quote,
    Dot,
}

type LexResult = Result<(String, WordState), LexError>;

/// Builds the current string based on a character and
/// adds it to the variable or operator deques when done
/// TODO: clean up this mess
fn build_string(build_type: StringBuildType,
                ch: char,
                variable_array: &mut VecDeque<AST>,
                operator_array: &mut VecDeque<String>,
                curr_state: &WordState,
                curr_str: String)
                -> LexResult {
    match build_type {
        StringBuildType::Letter => {
            match *curr_state {
                ref state @ WordState::Variable | ref state @ WordState::String =>
                    Ok((curr_str + ch.to_string().as_str(), state.clone())),
                WordState::Number | WordState::Decimal =>
                    Err(LexError::new("Number cannot have a letter after it")),
                WordState::Operator => {
                    operator_array.push_front(curr_str);
                    Ok((ch.to_string(), WordState::Variable))
                }
                WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
            }
        }
        StringBuildType::Digit => {
            match *curr_state {
                ref state @ WordState::Variable |
                ref state @ WordState::Number |
                ref state @ WordState::Decimal |
                ref state @ WordState::String =>
                    Ok((curr_str + ch.to_string().as_str(), state.clone())),
                WordState::Operator => {
                    operator_array.push_front(curr_str);
                    Ok((ch.to_string(), WordState::Number))
                }
                WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
            }
        }
        StringBuildType::Operator => {
            match *curr_state {
                WordState::Variable => {
                    variable_array.push_front(AST::Variable(curr_str));
                    Ok((ch.to_string(), WordState::Operator))
                }
                WordState::Number => {
                    variable_array.push_front(AST::Number(curr_str.as_str().parse().unwrap()));
                    Ok((ch.to_string(), WordState::Operator))
                }
                WordState::Decimal => {
                    variable_array.push_front(AST::Decimal(curr_str.as_str().parse().unwrap()));
                    Ok((ch.to_string(), WordState::Operator))
                }
                WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
                WordState::Operator => {
                    let new_str;
                    if is_operator((curr_str.clone() + ch.to_string().as_str()).as_str()) {
                        new_str = curr_str + ch.to_string().as_str();
                    } else {
                        operator_array.push_front(curr_str);
                        new_str = ch.to_string();
                    }

                    Ok((new_str, WordState::Operator))
                }
                WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Operator)),
            }
        }
        StringBuildType::Quote => {
            match *curr_state {
                WordState::String => {
                    variable_array.push_front(AST::String(curr_str));
                    Ok((String::new(), WordState::None))
                }
                WordState::Number | WordState::Decimal | WordState::Variable =>
                    Err(LexError::new("Cannot create a string after invalid type")),
                WordState::Operator => {
                    operator_array.push_front(curr_str);
                    Ok((String::new(), WordState::String))
                }
                WordState::None => Ok((curr_str, WordState::String)),
            }
        }
        StringBuildType::Dot => {
            match *curr_state {
                WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
                WordState::Number => Ok((curr_str + ch.to_string().as_str(), WordState::Decimal)),
                WordState::Operator | WordState::Decimal | WordState::Variable =>
                    Err(LexError::new("Cannot have a dot after")),
                WordState::None => Err(LexError::new("Cannot start with dot")),
            }
        }
    }
}

fn get_build_type(ch: char) -> StringBuildType {
    if ch.is_alphabetic() || ch == '_' {
        StringBuildType::Letter
    } else if ch.is_digit(10) {
        StringBuildType::Digit
    } else if ch == '\"' {
        StringBuildType::Quote
    } else if ch == '.' {
        StringBuildType::Dot
    } else {
        StringBuildType::Operator
    }
}

/// Parses a word split from split_string and detects operators 
/// Returns a deque of variables/constants and a deque of operators
/// TODO: clean up this mess
pub fn parse_string(s: &str) -> Result<(VecDeque<AST>, VecDeque<String>), LexError> {
    let mut variable_array: VecDeque<AST> = VecDeque::new();
    let mut operator_array = VecDeque::new();

    let mut curr_state = WordState::None;
    let mut curr_str = String::new();

    for ch in s.to_owned().chars() {
        if ch == ' ' || ch == '\t' {
            // ignore spaces and tabs except for inside a string
            if curr_state == WordState::String {
                curr_str = curr_str + ch.to_string().as_str();
            }
            continue;
        }

        match build_string(get_build_type(ch),
                           ch,
                           &mut variable_array,
                           &mut operator_array,
                           &curr_state,
                           curr_str) {
            Ok((new_str, new_state)) => {
                curr_str = new_str;
                curr_state = new_state;
            }
            Err(e) => return Err(e),
        }
    }

    // after string is finished, add the currently built string into the result
    if !curr_str.is_empty() {
        match curr_state {
            WordState::Variable => variable_array.push_front(AST::Variable(curr_str)),
            WordState::Number =>
                variable_array.push_front(AST::Number(curr_str.as_str().parse::<i32>().unwrap())),
            WordState::Decimal =>
                variable_array.push_front(AST::Decimal(curr_str.as_str().parse::<f64>().unwrap())),
            WordState::String => variable_array.push_front(AST::String(curr_str)),
            WordState::Operator => operator_array.push_front(curr_str),
            WordState::None => {}
        }
    }

    Ok((variable_array, operator_array))
}

#[test]
fn test_no_paren() {
    let s = "a+2-b+3";
    match parse_string(s) {
        Ok((variables, operators)) => {
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_owned()),
                                                      AST::Number(3)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["+", "-", "+"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_owned())
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
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_owned()),
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
                                                        .map(|s| s.to_owned())
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
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_owned()),
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
                                                        .map(|s| s.to_owned())
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
            let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                      AST::Number(2),
                                                      AST::Variable("b".to_owned()),
                                                      AST::Number(2),
                                                      AST::Number(5)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "+", "-", "^", ")", "=="]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_owned())
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
            let variable_result: VecDeque<AST> = vec![AST::String("Hello world1234 + ".to_owned()),
                                                      AST::String("bye123".to_owned())]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);

            let operator_result: VecDeque<String> = vec!["(", "+", ")"]
                                                        .into_iter()
                                                        .rev()
                                                        .map(|s| s.to_owned())
                                                        .collect();
            assert_eq!(operators, operator_result);
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_float() {
    let s = "1.23 - 3.12 + 123.45678";
    match parse_string(s) {
        Ok((variables, _)) => {
            let variable_result: VecDeque<AST> = vec![AST::Decimal(1.23),
                                                      AST::Decimal(3.12),
                                                      AST::Decimal(123.45678)]
                                                     .into_iter()
                                                     .rev()
                                                     .collect();
            assert_eq!(variables, variable_result);
        }
        Err(e) => println!("{:?}", e),
    }
}
