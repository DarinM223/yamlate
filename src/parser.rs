use ast::AST;
use helpers::operator_precedence;
use std::collections::VecDeque;

/// Parses string into AST
pub struct Parser {
    var_stack: VecDeque<AST>,
    op_stack: VecDeque<String>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            var_stack: VecDeque::new(),
            op_stack: VecDeque::new(),
        }
    }

    /// collapse_stacks combines the current variable and operator stacks into an AST
    /// and pushes the result back onto the variable stack
    /// TODO: Implement this
    fn collapse_stacks(&mut self) {
        while !self.var_stack.is_empty() && !self.op_stack.is_empty() {
            let variable = self.var_stack.pop_front();
            let operator = self.op_stack.pop_front();


        }
    }

    /// Takes in two deques for the variables and operators
    /// and returns the parsed AST value
    pub fn parse_to_ast(&mut self,
                        variables: &mut VecDeque<AST>,
                        operators: &mut VecDeque<String>)
                        -> Option<AST> {
        while !variables.is_empty() && !operators.is_empty() {
            self.var_stack.push_front(variables.pop_back().unwrap());

            let mut lower_precedence = false;

            // check if the next operator has greater precedence than the
            // current operator on the stack
            let operator = operators.pop_back().unwrap();

            {
                let front_operator = self.op_stack.front().unwrap();
                if operator_precedence(operator.as_str()) <
                   operator_precedence(front_operator.as_str()) {
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
