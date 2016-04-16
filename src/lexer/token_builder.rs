use ast::{Exp, Lit, Op};
use errors::LexError;
use helpers::is_operator;
use lexer::{LexerState, WordState};

/// TokenBuilder is used by the lexer to apply certain actions to the LexerState through
/// adding a new character
pub trait TokenBuilder {
    /// append adds a character from the parsed string, changing the lexer state depending
    /// on the type of character
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), LexError>;
}

// Implementations of TokenBuilder for handling letters,
// digits, operators, quotes, and dots
//

pub struct LetterBuilder;
pub struct DigitBuilder;
pub struct OperatorBuilder;
pub struct QuoteBuilder;
pub struct DotBuilder;

impl TokenBuilder for LetterBuilder {
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), LexError> {
        match state.curr_state {
            WordState::Variable |
            WordState::String => state.curr_chars.push(ch),

            WordState::Number |
            WordState::Decimal => return Err(LexError::new("Number cannot have a letter after it")),

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
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), LexError> {
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
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), LexError> {
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
                    _ => return Err(LexError::new("Invalid word state")),
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
    fn append(&self, _ch: char, state: &mut LexerState) -> Result<(), LexError> {
        match state.curr_state {
            WordState::String => {
                let curr_str = state.emit_string();
                state.variables.push_front(Exp::Lit(Lit::Str(curr_str)));
                state.curr_state = WordState::None;
            }

            WordState::Number |
            WordState::Decimal |
            WordState::Variable => {
                return Err(LexError::new("Cannot create a string after invalid type"))
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
    fn append(&self, ch: char, state: &mut LexerState) -> Result<(), LexError> {
        match state.curr_state {
            WordState::String => state.curr_chars.push(ch),

            WordState::Number => {
                state.curr_chars.push(ch);
                state.curr_state = WordState::Decimal;
            }

            WordState::Operator |
            WordState::Decimal |
            WordState::Variable => return Err(LexError::new("Cannot have a dot after")),

            WordState::None => return Err(LexError::new("Cannot start with dot")),
        }

        Ok(())
    }
}

pub fn append_ch(ch: char, state: &mut LexerState) -> Result<(), LexError> {
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
