use ast::AST;
use errors::LexError;
use helpers::{operator_precedence, operator_to_ast};
use std::collections::VecDeque;

/// try_op_unwrap is a macro that takes in an option of a string and
/// attempts to unwrap the option. If it can't, then the enclosing function
/// returns an error with the second string parameter 
macro_rules! try_op_unwrap {
    ($a:expr, $b:expr) => (match $a {
        Some(val) => val,
        None => return Some(LexError::new($b)),
    });
}

/// try_ast_unwrap is a macro that takes in an option of an AST and attempts
/// to unwrap the option. If it can't, then the enclosing function returns
/// an error with the second string parameter 
macro_rules! try_ast_unwrap {
    ($a:expr, $b:expr) => (match $a {
        Some(val) => if val == AST::None {
            return Some(LexError::new($b));
        } else {
            val
        },
        None => return Some(LexError::new($b)),
    });
}

const OPERATOR_ERROR: &'static str = "Operator cannot be retrieved from the stack";
const VARIABLE_ERROR: &'static str = "Variable cannot be retrieved from the stack";

/// Parses string into AST
pub struct Parser {
    var_stack: VecDeque<AST>,
    op_stack: VecDeque<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            var_stack: VecDeque::new(),
            op_stack: VecDeque::new(),
        }
    }

    /// collapse_stacks combines the current variable and operator stacks into an AST
    /// and pushes the result back onto the variable stack
    fn collapse_stacks(&mut self, add_op_precedence: i32) -> Option<LexError> {
        let mut paren_count = 0;
        while !self.op_stack.is_empty() &&
              (add_op_precedence < operator_precedence(self.op_stack.front().unwrap()) ||
               paren_count > 0) {
            let operator = try_op_unwrap!(self.op_stack.pop_front(), OPERATOR_ERROR);

            match operator.as_str() {
                ")" => paren_count += 1,
                "(" => paren_count -= 1, 

                "!" => {
                    let var = try_ast_unwrap!(self.var_stack.pop_front(), VARIABLE_ERROR);
                    self.var_stack.push_front(AST::Not(Box::new(var)));
                }

                // Double parameter operators
                _ => {
                    let var1 = try_ast_unwrap!(self.var_stack.pop_front(), VARIABLE_ERROR);
                    let var2 = try_ast_unwrap!(self.var_stack.pop_front(), VARIABLE_ERROR);

                    let ast_node = operator_to_ast(operator.as_str(), var2, var1);

                    self.var_stack.push_front(ast_node);
                }
            }
        }

        if paren_count != 0 {
            Some(LexError::new("Parentheses do not match"))
        } else {
            None
        }
    }

    /// Takes in two deques for the variables and operators
    /// and returns the parsed AST value
    pub fn parse_to_ast(&mut self,
                        variables: &mut VecDeque<AST>,
                        operators: &mut VecDeque<String>)
                        -> Result<AST, LexError> {
        while !operators.is_empty() {
            let mut lower_precedence = false;
            let mut op_precedence = -2;

            // check if the next operator has greater precedence than the
            // current operator on the stack
            // an operator with precedence -1 ignores precedence rules
            let operator = operators.pop_back().unwrap();

            if !self.op_stack.is_empty() {
                if let Some(front_operator) = self.op_stack.front() {
                    if operator_precedence(operator.as_str()) <
                       operator_precedence(front_operator.as_str()) &&
                       operator_precedence(operator.as_str()) != -1 {
                        lower_precedence = true;
                        op_precedence = operator_precedence(operator.as_str());
                    }
                }
            }

            if lower_precedence {
                let collapse_result = self.collapse_stacks(op_precedence);
                if let Some(err) = collapse_result {
                    return Err(err);
                }
            }

            if !variables.is_empty() && operator != "(" && operator != ")" {
                self.var_stack.push_front(variables.pop_back().unwrap());
            }

            self.op_stack.push_front(operator);
        }

        // Push the remaining variables at the end
        while !variables.is_empty() {
            self.var_stack.push_front(variables.pop_back().unwrap());
        }

        if !self.op_stack.is_empty() {
            let collapse_result = self.collapse_stacks(-2);
            if let Some(err) = collapse_result {
                return Err(err);
            }
        }

        if self.var_stack.len() > 1 {
            Err(LexError::new("Expression could not be completely evaluated"))
        } else {
            Ok(self.var_stack.pop_front().unwrap_or(AST::None))
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::AST;
    use errors::LexError;
    use std::collections::VecDeque;
    use super::*;

    #[test]
    fn test_collapse_stacks() {
        // test ast generation for `1 * (2 + 3)`
        // expected result:
        //      *
        //    /   \
        //   1     +
        //        / \
        //       2   3

        let mut parser = Parser::new();

        parser.var_stack.push_front(AST::Number(1));
        parser.var_stack.push_front(AST::Number(2));
        parser.var_stack.push_front(AST::Number(3));

        parser.op_stack.push_front("*".to_owned());
        parser.op_stack.push_front("(".to_owned());
        parser.op_stack.push_front("+".to_owned());
        parser.op_stack.push_front(")".to_owned());

        assert_eq!(parser.collapse_stacks(-2), None);

        assert_eq!(parser.var_stack.len(), 1);

        let expected_val = Some(AST::Times(box AST::Number(1),
                                           box AST::Plus(box AST::Number(2), box AST::Number(3))));

        assert_eq!(parser.var_stack.pop_front(), expected_val);
    }

    #[test]
    fn test_parse_error_right() {
        // test ast generation for `1 +`
        // should return error

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(AST::Number(1));

        operators.push_front("+".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        assert_eq!(result,
                   Err(LexError::new("Variable cannot be retrieved from the stack")));
    }

    #[test]
    fn test_parse_error_left() {
        // test ast generation for `+ 1`
        // should return error

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(AST::None);
        variables.push_front(AST::Number(1));

        operators.push_front("+".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        assert_eq!(result,
                   Err(LexError::new("Variable cannot be retrieved from the stack")));
    }

    #[test]
    fn test_parse_error_operator() {
        // test ast generation for `1 2`
        // should return error

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(AST::Number(1));
        variables.push_front(AST::Number(2));

        let result = parser.parse_to_ast(&mut variables, &mut operators);
        assert_eq!(result,
                   Err(LexError::new("Expression could not be completely evaluated")));
    }

    #[test]
    fn test_parse_to_ast() {
        // test ast generation for `1 + !5 ^ (2 && 6) * 2`
        // expected result:
        //     +
        //   /   \
        //  1     *
        //      /   \
        //    ^       2
        //  /   \
        // !     &&
        // |    /  \
        // 5   2    6

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(AST::Number(1));
        variables.push_front(AST::Number(5));
        variables.push_front(AST::Number(2));
        variables.push_front(AST::Number(6));
        variables.push_front(AST::Number(2));

        operators.push_front("+".to_owned());
        operators.push_front("!".to_owned());
        operators.push_front("^".to_owned());
        operators.push_front("(".to_owned());
        operators.push_front("&&".to_owned());
        operators.push_front(")".to_owned());
        operators.push_front("*".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        let expected_val =
            Ok(AST::Plus(box AST::Number(1),
                         box AST::Times(box AST::Exponent(box AST::Not(box AST::Number(5)),
                                                          box AST::And(box AST::Number(2),
                                                                       box AST::Number(6))),
                                        box AST::Number(2))));

        assert_eq!(result, expected_val);
    }

    #[test]
    fn test_parse_to_ast_simple() {
        // test ast generation for `1 + 2`
        // expected result:
        //    +
        //   / \
        //  1   2

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(AST::Number(1));
        variables.push_front(AST::Number(2));

        operators.push_front("+".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        let expected_val = Ok(AST::Plus(box AST::Number(1), box AST::Number(2)));

        assert_eq!(result, expected_val);
    }
}
