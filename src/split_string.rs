use ast::AST;
use helpers::is_operator;

enum WordState {
    Variable,
    Number,
    String,
    Operator,
    None,
}

fn split_string_letter(ch: char,
                       variable_array: &mut Vec<AST>,
                       operator_array: &mut Vec<String>,
                       curr_state: &WordState,
                       curr_str: String)
                       -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::Variable => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
        &WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Number => Err("Number cannot have a letter after it".to_string()),
        &WordState::Operator => {
            operator_array.push(curr_str);
            Ok((ch.to_string(), WordState::Variable))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
    }
}

fn split_string_digit(ch: char,
                      variable_array: &mut Vec<AST>,
                      operator_array: &mut Vec<String>,
                      curr_state: &WordState,
                      curr_str: String)
                      -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::Variable => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
        &WordState::Number => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
        &WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Operator => {
            operator_array.push(curr_str);
            Ok((ch.to_string(), WordState::Number))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
    }
}

fn split_string_operator(ch: char,
                         variable_array: &mut Vec<AST>,
                         operator_array: &mut Vec<String>,
                         curr_state: &WordState,
                         curr_str: String)
                         -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::Variable => {
            variable_array.push(AST::Variable(curr_str));
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::Number => {
            variable_array.push(AST::Number(curr_str));
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::String => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Operator => {
            let new_str;
            if is_operator((curr_str.clone() + ch.to_string().as_str()).as_str()) {
                new_str = curr_str + ch.to_string().as_str();
            } else {
                operator_array.push(curr_str);
                new_str = ch.to_string();
            }

            Ok((new_str, WordState::Operator))
        }
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Operator)),
    }
}

fn split_string_quote(ch: char,
                      variable_array: &mut Vec<AST>,
                      operator_array: &mut Vec<String>,
                      curr_state: &WordState,
                      curr_str: String)
                      -> Result<(String, WordState), String> {
    match curr_state {
        &WordState::String => {
            variable_array.push(AST::String(curr_str));
            Ok((String::new(), WordState::None))
        }
        &WordState::Number | &WordState::Variable =>
            Err("Cannot create a string after invalid type".to_string()),
        &WordState::Operator => {
            operator_array.push(curr_str);
            Ok((String::new(), WordState::String))
        }
        &WordState::None => Ok((curr_str, WordState::String)),
    }
}

/// Parses a word split from split_string and detects operators 
/// Returns an array of variables/constants and an array of operators
/// TODO: clean up this mess
pub fn split_string(s: &str) -> Result<(Vec<AST>, Vec<String>), String> {
    let mut variable_array: Vec<AST> = Vec::new();
    let mut operator_array = Vec::new();

    let mut curr_state = WordState::None;
    let mut curr_str = String::new();

    for ch in s.to_string().chars() {
        let split_fn: fn(char, &mut Vec<AST>, &mut Vec<String>, &WordState, String)
    -> Result<(String, WordState), String>;

        if ch.is_alphabetic() {
            split_fn = split_string_letter;
        } else if ch.is_digit(10) {
            split_fn = split_string_digit;
        } else if ch == ' ' || ch == '\t' {
            // ignore spaces and tabs except for inside a string
            match curr_state {
                WordState::String => {
                    curr_str = curr_str + ch.to_string().as_str();
                    continue;
                }
                _ => continue,
            }
        } else if ch == '\"' {
            split_fn = split_string_quote;
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
            WordState::Variable => variable_array.push(AST::Variable(curr_str)),
            WordState::Number => variable_array.push(AST::Number(curr_str)),
            WordState::String => variable_array.push(AST::String(curr_str)),
            WordState::Operator => operator_array.push(curr_str),
            WordState::None => {}
        }
    }

    return Ok((variable_array, operator_array));
}

#[test]
fn test_no_paren() {
    let s = "a+2-b+3";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec![AST::Variable("a".to_string()),
                                       AST::Number("2".to_string()),
                                       AST::Variable("b".to_string()),
                                       AST::Number("3".to_string())];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i]);
            }
            let operator_result = vec!["+", "-", "+"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i]);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_paren() {
    let s = "(a+(2-b)+(3*5))";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec![AST::Variable("a".to_string()),
                                       AST::Number("2".to_string()),
                                       AST::Variable("b".to_string()),
                                       AST::Number("3".to_string()),
                                       AST::Number("5".to_string())];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i]);
            }
            let operator_result = vec!["(", "+", "(", "-", ")", "+", "(", "*", ")", ")"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i]);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_equals() {
    let s = "(a==(2-b)+(3!=5))";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec![AST::Variable("a".to_string()),
                                       AST::Number("2".to_string()),
                                       AST::Variable("b".to_string()),
                                       AST::Number("3".to_string()),
                                       AST::Number("5".to_string())];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i]);
            }
            let operator_result = vec!["(", "==", "(", "-", ")", "+", "(", "!=", ")", ")"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i]);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_spaces() {
    let s = "( a + 2 - \t b \t^ 2 ) == 5";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec![AST::Variable("a".to_string()),
                                       AST::Number("2".to_string()),
                                       AST::Variable("b".to_string()),
                                       AST::Number("2".to_string()),
                                       AST::Number("5".to_string())];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i]);
            }
            let operator_result = vec!["(", "+", "-", "^", ")", "=="];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i]);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_strings() {
    let s = "( \"Hello world1234 + \" + \"bye123\" )";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec![AST::String("Hello world1234 + ".to_string()),
                                       AST::String("bye123".to_string())];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i]);
            }
            let operator_result = vec!["(", "+", ")"];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i]);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
