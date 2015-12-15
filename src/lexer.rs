use ast::AST;
use std::collections::VecDeque;
use errors::LexError;
use token_builder::builder_from_ch;

#[derive(Clone, PartialEq)]
pub enum WordState {
    Variable,
    Number,
    Decimal,
    String,
    Operator,
    None,
}

pub struct LexerState {
    pub variables: VecDeque<AST>,
    pub operators: VecDeque<String>,
    pub curr_state: WordState,
    pub curr_chars: Vec<char>,
}

impl LexerState {
    pub fn new() -> Self {
        LexerState {
            variables: VecDeque::new(),
            operators: VecDeque::new(),
            curr_state: WordState::None,
            curr_chars: Vec::new(),
        }
    }

    /// emit_string clears all of the current characters in the state
    /// and returns the string representation of the character array
    pub fn emit_string(&mut self) -> String {
        let curr_str = self.curr_chars.iter().cloned().collect::<String>();
        self.curr_chars.retain(|_| false);

        curr_str
    }
}


pub struct Lexer {
    pub state: LexerState,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { state: LexerState::new() }
    }

    /// parse_string parses the contents of the string and adds the
    /// tokens to the lexer. You can retrieve the tokenized
    /// operators through self.state.operators, and the tokenized
    /// AST constants/variables through self.state.variables
    pub fn parse_string(&mut self, s: &str) -> Option<LexError> {
        for ch in s.to_owned().chars() {
            if ch == ' ' || ch == '\t' {
                if self.state.curr_state == WordState::String {
                    self.state.curr_chars.push(ch);
                }
            } else {
                let builder = builder_from_ch(&ch);
                let result = builder.append(ch, &mut self.state);
                if let Some(err) = result {
                    return Some(err);
                }
            }
        }

        // after string is finished, add the currently built string into the result
        if !self.state.curr_chars.is_empty() {
            let curr_str = self.state.emit_string();

            if self.state.curr_state == WordState::Operator {
                self.state.operators.push_front(curr_str);
            } else if self.state.curr_state != WordState::None {
                let ast_node = match self.state.curr_state {
                    WordState::Variable => AST::Variable(curr_str),
                    WordState::Number => AST::Number(curr_str.as_str().parse().unwrap_or(0)),
                    WordState::Decimal => AST::Decimal(curr_str.as_str().parse().unwrap_or(0.0)),
                    WordState::String => AST::String(curr_str),
                    _ => AST::None,
                };

                self.state.variables.push_front(ast_node);
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use ast::AST;
    use std::collections::VecDeque;
    use super::*;

    #[test]
    fn test_no_paren() {
        let s = "a+2-b+3";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                  AST::Number(2),
                                                  AST::Variable("b".to_owned()),
                                                  AST::Number(3)]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);

        let operator_result: VecDeque<String> = vec!["+", "-", "+"]
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| s.to_owned())
                                                    .collect();
        assert_eq!(lexer.state.operators, operator_result);
    }

    #[test]
    fn test_paren() {
        let s = "(a+(2-b)+(3*5))";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                  AST::Number(2),
                                                  AST::Variable("b".to_owned()),
                                                  AST::Number(3),
                                                  AST::Number(5)]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);

        let operator_result: VecDeque<String> = vec!["(", "+", "(", "-", ")", "+", "(", "*", ")",
                                                     ")"]
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| s.to_owned())
                                                    .collect();
        assert_eq!(lexer.state.operators, operator_result);
    }

    #[test]
    fn test_equals() {
        let s = "(a==(2-b)+(3!=5))";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                  AST::Number(2),
                                                  AST::Variable("b".to_owned()),
                                                  AST::Number(3),
                                                  AST::Number(5)]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);

        let operator_result: VecDeque<String> = vec!["(", "==", "(", "-", ")", "+", "(", "!=",
                                                     ")", ")"]
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| s.to_owned())
                                                    .collect();
        assert_eq!(lexer.state.operators, operator_result);
    }

    #[test]
    fn test_spaces() {
        let s = "( a + 2 - \t b \t^ 2 ) == 5";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::Variable("a".to_owned()),
                                                  AST::Number(2),
                                                  AST::Variable("b".to_owned()),
                                                  AST::Number(2),
                                                  AST::Number(5)]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);

        let operator_result: VecDeque<String> = vec!["(", "+", "-", "^", ")", "=="]
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| s.to_owned())
                                                    .collect();
        assert_eq!(lexer.state.operators, operator_result);
    }

    #[test]
    fn test_strings() {
        let s = "( \"Hello world1234 + \" + \"bye123\" )";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::String("Hello world1234 + ".to_owned()),
                                                  AST::String("bye123".to_owned())]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);

        let operator_result: VecDeque<String> = vec!["(", "+", ")"]
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| s.to_owned())
                                                    .collect();
        assert_eq!(lexer.state.operators, operator_result);
    }

    #[test]
    fn test_float() {
        let s = "1.23 - 3.12 + 123.45678";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), None);

        let variable_result: VecDeque<AST> = vec![AST::Decimal(1.23),
                                                  AST::Decimal(3.12),
                                                  AST::Decimal(123.45678)]
                                                 .into_iter()
                                                 .rev()
                                                 .collect();
        assert_eq!(lexer.state.variables, variable_result);
    }
}
