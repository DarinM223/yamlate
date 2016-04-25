use ast::{Exp, Op};
use errors::{LexError, YamlError};
use helpers::{operator_precedence, operator_to_exp};
use std::collections::VecDeque;

/// Parses string into AST
pub struct Parser {
    var_stack: VecDeque<Exp>,
    op_stack: VecDeque<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            var_stack: VecDeque::new(),
            op_stack: VecDeque::new(),
        }
    }

    /// Combines the current variable and operator stacks into an AST
    /// and pushes the result back onto the variable stack
    fn collapse_stacks(&mut self, add_op_precedence: i32) -> Result<(), YamlError> {
        let mut paren_count = 0;
        while !self.op_stack.is_empty() &&
              (add_op_precedence < operator_precedence(self.op_stack.front().unwrap()) ||
               paren_count > 0) {
            let operator = match self.op_stack.pop_front() {
                Some(op) => op,
                None => return Err(YamlError::LexError(LexError::OperatorStackError)),
            };

            match operator.as_str() {
                ")" => paren_count += 1,
                "(" => paren_count -= 1,

                "!" => {
                    let var = match self.var_stack.pop_front() {
                        Some(var) => var,
                        None => return Err(YamlError::LexError(LexError::VariableStackError)),
                    };
                    self.var_stack.push_front(Exp::UnaryOp(Op::Not, Box::new(var)));
                }

                // Double parameter operators
                _ => {
                    let var1 = match self.var_stack.pop_front() {
                        Some(var) => var,
                        None => return Err(YamlError::LexError(LexError::VariableStackError)),
                    };
                    let var2 = match self.var_stack.pop_front() {
                        Some(var) => var,
                        None => return Err(YamlError::LexError(LexError::VariableStackError)),
                    };
                    let ast_node = try!(operator_to_exp(operator.as_str(), var2, var1));

                    self.var_stack.push_front(ast_node);
                }
            }
        }

        if paren_count != 0 {
            Err(YamlError::LexError(LexError::ParenthesisNotMatch))
        } else {
            Ok(())
        }
    }

    /// Takes in two deques for the variables and operators
    /// and returns the parsed AST value
    pub fn parse_to_ast(&mut self,
                        variables: &mut VecDeque<Exp>,
                        operators: &mut VecDeque<String>)
                        -> Result<Exp, YamlError> {
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
                try!(self.collapse_stacks(op_precedence));
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
            try!(self.collapse_stacks(-2));
        }

        if self.var_stack.len() > 1 {
            Err(YamlError::LexError(LexError::Incomplete))
        } else if self.var_stack.len() == 1 {
            Ok(self.var_stack.pop_front().unwrap())
        } else {
            Err(YamlError::LexError(LexError::ResultNotLiteral))
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::{Exp, Lit, Op};
    use errors::{LexError, YamlError};
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

        parser.var_stack.push_front(Exp::Lit(Lit::Number(1)));
        parser.var_stack.push_front(Exp::Lit(Lit::Number(2)));
        parser.var_stack.push_front(Exp::Lit(Lit::Number(3)));

        parser.op_stack.push_front("*".to_owned());
        parser.op_stack.push_front("(".to_owned());
        parser.op_stack.push_front("+".to_owned());
        parser.op_stack.push_front(")".to_owned());

        assert_eq!(parser.collapse_stacks(-2), Ok(()));
        assert_eq!(parser.var_stack.len(), 1);

        let expected_val =
            Some(Exp::BinaryOp(Op::Times,
                               Box::new(Exp::Lit(Lit::Number(1))),
                               Box::new(Exp::BinaryOp(Op::Plus,
                                                      Box::new(Exp::Lit(Lit::Number(2))),
                                                      Box::new(Exp::Lit(Lit::Number(3)))))));

        assert_eq!(parser.var_stack.pop_front(), expected_val);
    }

    #[test]
    fn test_parse_error_right() {
        // test ast generation for `1 +`
        // should return error

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(Exp::Lit(Lit::Number(1)));

        operators.push_front("+".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        assert_eq!(result,
                   Err(YamlError::LexError(LexError::VariableStackError)));
    }

    #[test]
    fn test_parse_error_operator() {
        // test ast generation for `1 2`
        // should return error

        let mut parser = Parser::new();

        let mut variables = VecDeque::new();
        let mut operators = VecDeque::new();

        variables.push_front(Exp::Lit(Lit::Number(1)));
        variables.push_front(Exp::Lit(Lit::Number(2)));

        let result = parser.parse_to_ast(&mut variables, &mut operators);
        assert_eq!(result, Err(YamlError::LexError(LexError::Incomplete)));
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

        variables.push_front(Exp::Lit(Lit::Number(1)));
        variables.push_front(Exp::Lit(Lit::Number(5)));
        variables.push_front(Exp::Lit(Lit::Number(2)));
        variables.push_front(Exp::Lit(Lit::Number(6)));
        variables.push_front(Exp::Lit(Lit::Number(2)));

        operators.push_front("+".to_owned());
        operators.push_front("!".to_owned());
        operators.push_front("^".to_owned());
        operators.push_front("(".to_owned());
        operators.push_front("&&".to_owned());
        operators.push_front(")".to_owned());
        operators.push_front("*".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        let not_tree = Exp::UnaryOp(Op::Not, Box::new(Exp::Lit(Lit::Number(5))));
        let and_tree = Exp::BinaryOp(Op::And,
                                     Box::new(Exp::Lit(Lit::Number(2))),
                                     Box::new(Exp::Lit(Lit::Number(6))));
        let pow_tree = Exp::BinaryOp(Op::Exponent, Box::new(not_tree), Box::new(and_tree));
        let times_tree = Exp::BinaryOp(Op::Times,
                                       Box::new(pow_tree),
                                       Box::new(Exp::Lit(Lit::Number(2))));
        let expected_val = Exp::BinaryOp(Op::Plus,
                                         Box::new(Exp::Lit(Lit::Number(1))),
                                         Box::new(times_tree));

        assert_eq!(result, Ok(expected_val));
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

        variables.push_front(Exp::Lit(Lit::Number(1)));
        variables.push_front(Exp::Lit(Lit::Number(2)));

        operators.push_front("+".to_owned());

        let result = parser.parse_to_ast(&mut variables, &mut operators);

        let expected_val = Ok(Exp::BinaryOp(Op::Plus,
                                            Box::new(Exp::Lit(Lit::Number(1))),
                                            Box::new(Exp::Lit(Lit::Number(2)))));

        assert_eq!(result, expected_val);
    }
}
