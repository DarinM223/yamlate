use ast::AST;
use std::str::FromStr;
use std::collections::HashMap;

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Evaluator
    }

    /// evaluate evaluates the given AST and returns an AST 
    /// of the result 
    pub fn evaluate(&mut self, ast: AST) -> Option<AST> {
        match ast {
            // TODO: add environment support to Evaluator
            // &AST::Variable(name) => self.env.get(name),
            AST::Number(val) => Some(AST::Number(val)),
            AST::String(s) => Some(AST::String(s)),
            AST::Plus(box child1, box child2) => {
                let result1 = self.evaluate(child1);
                let result2 = self.evaluate(child2);

                let mut param1: i32 = 0;
                let mut param2: i32 = 0;

                match result1 { 
                    Some(AST::Number(val)) => param1 = val.as_str().parse().unwrap(),
                    _ => return None,
                }

                match result2 {
                    Some(AST::Number(val)) => param2 = val.as_str().parse().unwrap(),
                    _ => return None,
                }

                Some(AST::Number((param1 + param2).to_string()))
            }
            _ => Some(AST::None),
        }
    }
}
