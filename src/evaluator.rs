use ast::AST;
use environment::{IEnvironment, Environment};
use std::str::FromStr;
use std::collections::HashMap;

pub struct Evaluator<'a> {
    env: &'a mut IEnvironment,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut IEnvironment) -> Self {
        Evaluator { env: env }
    }

    fn arithmetic_operation(&mut self, operator: &str, child1: AST, child2: AST) -> Option<AST> {
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

        Some(AST::Number(apply_arithmetic_operator(operator, param1, param2).to_string()))
    }

    /// evaluate evaluates the given AST and returns an AST 
    /// of the result 
    pub fn evaluate(&mut self, ast: AST) -> Option<AST> {
        match ast {
            AST::Variable(name) => match self.env.get(name) {
                Some(val) => Some(val.clone()),
                None => None,
            },
            AST::Number(val) => Some(AST::Number(val)),
            AST::String(s) => Some(AST::String(s)),
            AST::Plus(box child1, box child2) => self.arithmetic_operation("+", child1, child2),
            AST::Minus(box child1, box child2) => self.arithmetic_operation("-", child1, child2),
            AST::Times(box child1, box child2) => self.arithmetic_operation("*", child1, child2),
            AST::Divide(box child1, box child2) => self.arithmetic_operation("/", child1, child2),
            AST::Modulo(box child1, box child2) => self.arithmetic_operation("%", child1, child2),
            _ => Some(AST::None),
        }
    }
}

fn apply_arithmetic_operator(operator: &str, a: i32, b: i32) -> i32 {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        _ => -1, 
    }
}

#[test]
fn test_arith_ast() {
    // Test that the result for ast:
    //      *
    //    /   \
    //   5     +
    //       /   \
    //      -     6
    //    /   \
    //   3     2
    // is "35"
    
    let mut env = Environment::new();
    let mut evaluator = Evaluator::new(&mut env);

    let ast = AST::Times(box AST::Number("5".to_string()),
                         box AST::Plus(box AST::Minus(box AST::Number("3".to_string()),
                                                      box AST::Number("2".to_string())),
                                       box AST::Number("6".to_string())));

    let result = evaluator.evaluate(ast);

    assert_eq!(result, Some(AST::Number("35".to_string())));
}
