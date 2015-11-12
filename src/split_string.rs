use helpers::is_operator;

enum WordState {
    Variable,
    Number,
    String,
    Operator,
    None,
}

fn split_string_letter(ch: char,
                       variable_array: &mut Vec<String>,
                       operator_array: &mut Vec<String>,
                       curr_state: &WordState, 
                       curr_str: String) -> Result<(String, WordState), String> {

    match curr_state {
        &WordState::Variable => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
        &WordState::String   => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Number   => Err("Number cannot have a letter after it".to_string()),
        &WordState::Operator => {
            operator_array.push(curr_str);
            Ok((ch.to_string(), WordState::Variable))
        },
        &WordState::None     => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
    }
}

fn split_string_digit(ch: char,
                      variable_array: &mut Vec<String>,
                      operator_array: &mut Vec<String>,
                      curr_state: &WordState, 
                      curr_str: String) -> Result<(String, WordState), String> {

    match curr_state {
        &WordState::Variable => Ok((curr_str + ch.to_string().as_str(), WordState::Variable)),
        &WordState::Number   => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
        &WordState::String   => Ok((curr_str + ch.to_string().as_str(), WordState::String)),
        &WordState::Operator => {
            operator_array.push(curr_str);
            Ok((ch.to_string(), WordState::Number))
        },
        &WordState::None     => Ok((curr_str + ch.to_string().as_str(), WordState::Number)),
    }
}

fn split_string_operator(ch: char, 
                         variable_array: &mut Vec<String>,
                         operator_array: &mut Vec<String>,
                         curr_state: &WordState, 
                         curr_str: String) -> Result<(String, WordState), String> {

    match curr_state {
        &WordState::Variable | &WordState::Number | &WordState::String => {
            variable_array.push(curr_str);
            Ok((ch.to_string(), WordState::Operator))
        }
        &WordState::Operator => {
            let new_str;
            if is_operator((curr_str.clone() + ch.to_string().as_str()).as_str()) {
                new_str = curr_str + ch.to_string().as_str();
            } else {
                operator_array.push(curr_str);
                new_str = ch.to_string();
            }

            Ok((new_str, WordState::Operator))
        },
        &WordState::None => Ok((curr_str + ch.to_string().as_str(), WordState::Operator)),
    }
}

/// Parses a word split from split_string and detects operators 
/// Returns an array of variables/constants and an array of operators
/// TODO: clean up this mess
pub fn split_string(s: &str) -> Result<(Vec<String>, Vec<String>), String> {
    let mut variable_array = Vec::new();
    let mut operator_array = Vec::new();

    let mut curr_state = WordState::None;
    let mut curr_str = String::new();

    for ch in s.to_string().chars() {
        let split_fn: fn(char, &mut Vec<String>, &mut Vec<String>, &WordState, String) -> Result<(String, WordState), String>;
        
        if ch.is_alphabetic() {
            split_fn = split_string_letter;
        } else if ch.is_digit(10) {
            split_fn = split_string_digit;
        } else if ch == ' ' || ch == '\t' { // ignore spaces and tabs
            continue;
        } else {
            split_fn = split_string_operator;
        }

        match split_fn(ch, &mut variable_array, &mut operator_array, &mut curr_state, curr_str) {
            Ok((new_str, new_state)) => {
                curr_str = new_str;
                curr_state = new_state;
            },
            Err(e) => return Err(e),
        }
    }

    // after string is finished, add the currently built string into the result
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
fn test_split_string_no_paren() {
    let s = "a+2-b+3";
    match split_string(s) {
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
fn test_split_string_paren() {
    let s = "(a+(2-b)+(3*5))";
    match split_string(s) {
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
fn test_split_string_equals() {
    let s = "(a==(2-b)+(3!=5))";
    match split_string(s) {
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

#[test]
fn test_split_string_spaces() {
    let s = "( a + 2 - \t b \t^ 2 ) == 5";
    match split_string(s) {
        Ok((variables, operators)) => {
            let variable_result = vec!["a", "2", "b", "2", "5"];
            for i in 0..variables.len() {
                assert_eq!(variables[i], variable_result[i].to_string());
            }
            let operator_result = vec!["(", "+", "-", "^", ")", "==" ];
            for i in 0..operators.len() {
                assert_eq!(operators[i], operator_result[i].to_string());
            }
        },
        Err(e) => println!("{:?}", e),
    }
}