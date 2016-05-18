mod token_builder;

use ast::{Exp, Lit};
use errors::{LexError, YamlError};
use lexer::token_builder::append_ch;
use std::collections::VecDeque;

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
    pub variables: VecDeque<Exp>,
    pub operators: VecDeque<String>,
    pub curr_state: WordState,
    pub curr_chars: Vec<char>,
}

impl LexerState {
    pub fn new() -> LexerState {
        LexerState {
            variables: VecDeque::new(),
            operators: VecDeque::new(),
            curr_state: WordState::None,
            curr_chars: Vec::new(),
        }
    }

    /// Clears all of the current characters in the state
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
    pub fn new() -> Lexer {
        Lexer { state: LexerState::new() }
    }

    /// Parses the contents of the string and adds the
    /// tokens to the lexer. You can retrieve the tokenized
    /// operators through self.state.operators, and the tokenized
    /// Exp constants/variables through self.state.variables
    pub fn parse_string(&mut self, s: &str) -> Result<(), YamlError> {
        for ch in s.to_owned().chars() {
            if ch == ' ' || ch == '\t' {
                if self.state.curr_state == WordState::String {
                    self.state.curr_chars.push(ch);
                }
            } else {
                try!(append_ch(ch, &mut self.state));
            }
        }

        // after string is finished, add the currently built string into the result
        if !self.state.curr_chars.is_empty() {
            let curr_str = self.state.emit_string();

            if self.state.curr_state == WordState::Operator {
                self.state.operators.push_front(curr_str);
            } else if self.state.curr_state != WordState::None {
                let ast_node = match self.state.curr_state {
                    WordState::Variable => Exp::Variable(curr_str),
                    WordState::Number => {
                        Exp::Lit(Lit::Number(curr_str.as_str().parse().unwrap_or(0)))
                    }
                    WordState::Decimal => {
                        Exp::Lit(Lit::Decimal(curr_str.as_str().parse().unwrap_or(0.0)))
                    }
                    WordState::String => Exp::Lit(Lit::Str(curr_str)),
                    _ => return Err(YamlError::LexError(LexError::ResultNotLiteral)),
                };

                self.state.variables.push_front(ast_node);
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use ast::{Exp, Lit};
    use std::collections::VecDeque;
    use super::*;

    #[test]
    fn test_no_paren() {
        let s = "a+2-b+3";
        let mut lexer = Lexer::new();
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Variable("a".to_owned()),
                                                  Exp::Lit(Lit::Number(2)),
                                                  Exp::Variable("b".to_owned()),
                                                  Exp::Lit(Lit::Number(3))]
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
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Variable("a".to_owned()),
                                                  Exp::Lit(Lit::Number(2)),
                                                  Exp::Variable("b".to_owned()),
                                                  Exp::Lit(Lit::Number(3)),
                                                  Exp::Lit(Lit::Number(5))]
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
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Variable("a".to_owned()),
                                                  Exp::Lit(Lit::Number(2)),
                                                  Exp::Variable("b".to_owned()),
                                                  Exp::Lit(Lit::Number(3)),
                                                  Exp::Lit(Lit::Number(5))]
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
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Variable("a".to_owned()),
                                                  Exp::Lit(Lit::Number(2)),
                                                  Exp::Variable("b".to_owned()),
                                                  Exp::Lit(Lit::Number(2)),
                                                  Exp::Lit(Lit::Number(5))]
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
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Lit(Lit::Str("Hello world1234 + "
                                                      .to_owned())),
                                                  Exp::Lit(Lit::Str("bye123".to_owned()))]
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
        assert_eq!(lexer.parse_string(s), Ok(()));

        let variable_result: VecDeque<Exp> = vec![Exp::Lit(Lit::Decimal(1.23)),
                                                  Exp::Lit(Lit::Decimal(3.12)),
                                                  Exp::Lit(Lit::Decimal(123.45678))]
            .into_iter()
            .rev()
            .collect();
        assert_eq!(lexer.state.variables, variable_result);
    }
}
