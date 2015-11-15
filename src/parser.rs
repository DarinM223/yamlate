use ast::AST;
use helpers::operator_precedence;
use std::collections::VecDeque;

/// Parses string into AST
pub struct Parser {
    var_stack: VecDeque<AST>,
    op_stack: VecDeque<String>,
    err: Result<bool, String>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            var_stack: VecDeque::new(),
            op_stack: VecDeque::new(),
            err: Ok(true),
        }
    }

    /// collapse_stacks combines the current variable and operator stacks into an AST
    /// and pushes the result back onto the variable stack
    fn collapse_stacks(&mut self) {
        let mut paren_count = 0;
        while !self.op_stack.is_empty() {
            let operator = self.op_stack.pop_front().unwrap();

            match operator.as_str() {
                ")" => paren_count += 1,
                "(" => paren_count -= 1, 

                "!" => {
                    let var = self.var_stack.pop_front().unwrap();
                    self.var_stack.push_front(AST::Not(Box::new(var)));
                },

                // Double parameter operators
                _ => {
                    let var1 = self.var_stack.pop_front().unwrap();
                    let var2 = self.var_stack.pop_front().unwrap();

                    let ast_node = match operator.as_str() {
                        "=" => AST::Assign(Box::new(var1), Box::new(var2)),
                        "+" => AST::Plus(Box::new(var1), Box::new(var2)),
                        "-" => AST::Minus(Box::new(var1), Box::new(var2)),
                        "*" => AST::Times(Box::new(var1), Box::new(var2)),
                        "/" => AST::Divide(Box::new(var1), Box::new(var2)),
                        "%" => AST::Modulo(Box::new(var1), Box::new(var2)),
                        "^" => AST::Exponent(Box::new(var1), Box::new(var2)),
                        "&&" => AST::And(Box::new(var1), Box::new(var2)),
                        "||" => AST::Or(Box::new(var1), Box::new(var2)),
                        _ => AST::None,
                    };

                    self.var_stack.push_front(ast_node);
                }
            }
        }

        if paren_count != 0 {
            self.err = Err("Parentheses do not match".to_string());
        }
    }

    /// Takes in two deques for the variables and operators
    /// and returns the parsed AST value
    pub fn parse_to_ast(&mut self,
                        variables: &mut VecDeque<AST>,
                        operators: &mut VecDeque<String>)
                        -> Option<AST> {
        while !operators.is_empty() {
            if !variables.is_empty() {
                self.var_stack.push_front(variables.pop_back().unwrap());
            }

            let mut lower_precedence = false;

            // check if the next operator has greater precedence than the
            // current operator on the stack
            // an operator with precedence -1 ignores precedence rules
            let operator = operators.pop_back().unwrap();

            if self.op_stack.len() > 0 {
                let front_operator = self.op_stack.front().unwrap();
                if operator_precedence(operator.as_str()) <
                   operator_precedence(front_operator.as_str()) &&
                   operator_precedence(operator.as_str()) != -1 {
                    lower_precedence = true;
                }
            }

            if lower_precedence {
                self.collapse_stacks();
            }

            self.op_stack.push_front(operator);
        }

        self.collapse_stacks();

        self.var_stack.pop_front()
    }
}

#[test]
fn test_collapse_stacks() {
    // test ast generation for `1 * (2 + 3)`

    let mut parser = Parser::new();

    parser.var_stack.push_front(AST::Number("1".to_string()));
    parser.var_stack.push_front(AST::Number("2".to_string()));
    parser.var_stack.push_front(AST::Number("3".to_string()));

    parser.op_stack.push_front("*".to_string());
    parser.op_stack.push_front("(".to_string());
    parser.op_stack.push_front("+".to_string());
    parser.op_stack.push_front(")".to_string());

    parser.collapse_stacks();

    assert_eq!(parser.var_stack.len(), 1);

    match parser.var_stack.pop_front().take() {
        Some(root) => {
            match root {
                AST::Times(box leaf1, box leaf2) => {
                    match leaf1 {
                        AST::Plus(box leaf1, box leaf2) => {
                            assert_eq!(leaf1, AST::Number("3".to_string()));
                            assert_eq!(leaf2, AST::Number("2".to_string()));
                        }
                        _ => {
                            println!("Left child node is not a plus operator");
                            assert!(false);
                        }
                    }

                    assert_eq!(leaf2, AST::Number("1".to_string()));
                }
                _ => {
                    println!("Root node is not a times operator");
                    assert!(false);
                }
            }
        }
        None => {
            println!("Failed to unparse the first element");
            assert!(false);
        }
    }
}

#[test]
fn test_parse_to_ast() {
    // test ast generation for `1 + !5 ^ (2 && 6) * 2`
    let mut parser = Parser::new();

    let mut variables = VecDeque::new();
    let mut operators = VecDeque::new();

    variables.push_front(AST::Number("1".to_string()));
    variables.push_front(AST::Number("5".to_string()));
    variables.push_front(AST::Number("2".to_string()));
    variables.push_front(AST::Number("6".to_string()));
    variables.push_front(AST::Number("2".to_string()));

    operators.push_front("+".to_string());
    operators.push_front("!".to_string());
    operators.push_front("^".to_string());
    operators.push_front("(".to_string());
    operators.push_front("&&".to_string());
    operators.push_front(")".to_string());
    operators.push_front("*".to_string());

    let result = parser.parse_to_ast(&mut variables, &mut operators);

    println!("{:?}", result);
}
