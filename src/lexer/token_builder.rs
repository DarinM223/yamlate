use ast::{Exp, Lit};
use errors::{LexError, YamlError};
use helpers::is_operator;
use lexer::{LexerState, WordState};

/// TokenBuilder is used by the lexer to apply certain actions to the LexerState through
/// adding a new character
pub trait TokenBuilder {
    /// append adds a character from the parsed string, changing the lexer state depending
    /// on the type of character
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), YamlError>;
}

// Implementations of TokenBuilder for handling letters,
// digits, operators, quotes, and dots

pub struct LetterBuilder;
pub struct DigitBuilder;
pub struct OperatorBuilder;
pub struct QuoteBuilder;
pub struct DotBuilder;

impl TokenBuilder for LetterBuilder {
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), YamlError> {
        match state.curr_state {
            WordState::Variable |
            WordState::String => state.curr_chars.push(ch),

            WordState::Number |
            WordState::Decimal => return Err(YamlError::LexError(LexError::LetterAfterNumber)),

            WordState::Operator => {
                let curr_str = state.emit_string();
                state.operators.push_front(curr_str);
                state.curr_chars.push(ch);
                state.curr_state = WordState::Variable;
            }

            WordState::None => {
                state.curr_chars.push(ch);
                state.curr_state = WordState::Variable;
            }
        }

        Ok(())
    }
}

impl TokenBuilder for DigitBuilder {
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), YamlError> {
        match state.curr_state {
            WordState::Variable |
            WordState::Number |
            WordState::Decimal |
            WordState::String => state.curr_chars.push(ch),

            WordState::Operator => {
                let curr_str = state.emit_string();
                state.operators.push_front(curr_str);
                state.curr_chars.push(ch);
                state.curr_state = WordState::Number;
            }

            WordState::None => {
                state.curr_chars.push(ch);
                state.curr_state = WordState::Number;
            }
        }

        Ok(())
    }
}

impl TokenBuilder for OperatorBuilder {
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), YamlError> {
        match state.curr_state {
            WordState::Variable |
            WordState::Number |
            WordState::Decimal => {
                let curr_str = state.emit_string();
                let ast_node = match state.curr_state {
                    WordState::Variable => Exp::Variable(curr_str),
                    WordState::Number => {
                        Exp::Lit(Lit::Number(curr_str.as_str().parse().unwrap_or(0)))
                    }
                    WordState::Decimal => {
                        Exp::Lit(Lit::Decimal(curr_str.as_str().parse().unwrap_or(0.0)))
                    }
                    _ => return Err(YamlError::LexError(LexError::ResultNotLiteral)),
                };

                state.variables.push_front(ast_node);
                state.curr_chars.push(ch);
                state.curr_state = WordState::Operator;
            }

            WordState::String => state.curr_chars.push(ch),

            WordState::Operator => {
                state.curr_chars.push(ch);
                let op_str = state.curr_chars.iter().cloned().collect::<String>();

                if !is_operator(op_str.as_str()) {
                    state.curr_chars.pop();
                    let curr_str = state.emit_string();
                    state.operators.push_front(curr_str);
                    state.curr_chars.push(ch);
                }
            }

            WordState::None => {
                state.curr_chars.push(ch);
                state.curr_state = WordState::Operator;
            }
        }

        Ok(())
    }
}

impl TokenBuilder for QuoteBuilder {
    fn append(&self, _ch: char, state: &mut LexerState) -> Result<(), YamlError> {
        match state.curr_state {
            WordState::String => {
                let curr_str = state.emit_string();
                state.variables.push_front(Exp::Lit(Lit::Str(curr_str)));
                state.curr_state = WordState::None;
            }

            WordState::Number |
            WordState::Decimal |
            WordState::Variable => {
                return Err(YamlError::LexError(LexError::InvalidQuoteAppend));
            }

            WordState::Operator => {
                let curr_str = state.emit_string();
                state.operators.push_front(curr_str);
                state.curr_state = WordState::String;
            }

            WordState::None => state.curr_state = WordState::String,
        }

        Ok(())
    }
}

impl TokenBuilder for DotBuilder {
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), YamlError> {
        match state.curr_state {
            WordState::String => state.curr_chars.push(ch),

            WordState::Number => {
                state.curr_chars.push(ch);
                state.curr_state = WordState::Decimal;
            }

            WordState::Operator |
            WordState::Decimal |
            WordState::Variable |
            WordState::None => return Err(YamlError::LexError(LexError::InvalidDotAppend)),
        }

        Ok(())
    }
}

pub fn append_ch(ch: char, state: &mut LexerState) -> Result<(), YamlError> {
    if ch.is_alphabetic() || ch == '_' {
        LetterBuilder.append(ch, state)
    } else if ch.is_digit(10) {
        DigitBuilder.append(ch, state)
    } else if ch == '\"' {
        QuoteBuilder.append(ch, state)
    } else if ch == '.' {
        DotBuilder.append(ch, state)
    } else {
        OperatorBuilder.append(ch, state)
    }
}
