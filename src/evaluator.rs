extern crate num;

use ast::AST;
use environment::{IEnvironment, Environment};
use std::collections::HashMap;
use helpers::ast_to_operator;
use self::num::traits::Num;

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

        match result1 {
            Some(AST::Number(val)) => {
                let mut param1: i32 = val;

                match result2 {
                    Some(AST::Decimal(param2)) =>
                        Some(AST::Decimal(apply_arithmetic_operator(operator,
                                                                    param1 as f64,
                                                                    param2))),
                    Some(AST::Number(param2)) =>
                        Some(AST::Number(apply_arithmetic_operator(operator, param1, param2))),
                    _ => None,
                }
            }
            Some(AST::Decimal(val)) => {
                let mut param1: f64 = val;

                match result2 {
                    Some(AST::Number(param2)) =>
                        Some(AST::Decimal(apply_arithmetic_operator(operator,
                                                                    param1,
                                                                    param2 as f64))),
                    Some(AST::Decimal(param2)) =>
                        Some(AST::Decimal(apply_arithmetic_operator(operator, param1, param2))),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// evaluate evaluates the given AST and returns an AST 
    /// of the result 
    pub fn evaluate(&mut self, ast: AST) -> Option<AST> {
        let op = ast_to_operator(&ast);
        match ast {
            AST::Variable(name) => match self.env.get(name) {
                Some(val) => Some(val.clone()),
                None => None,
            },
            ast @ AST::Number(_) | ast @ AST::String(_) | ast @ AST::Decimal(_) => Some(ast),
            AST::Plus(box child1, box child2) |
            AST::Minus(box child1, box child2) |
            AST::Times(box child1, box child2) |
            AST::Divide(box child1, box child2) |
            AST::Modulo(box child1, box child2) =>
                self.arithmetic_operation(op.as_str(), child1, child2),
            _ => Some(AST::None),
        }
    }
}

fn apply_arithmetic_operator<T: Num>(operator: &str, a: T, b: T) -> T {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        _ => a + b, 
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

    let ast = AST::Times(box AST::Number(5),
                         box AST::Plus(box AST::Minus(box AST::Number(3), box AST::Number(2)),
                                       box AST::Number(6)));

    let result = evaluator.evaluate(ast);

    assert_eq!(result, Some(AST::Number(35)));
}

#[test]
fn test_variable_ast() {
    // Test that the result for ast:
    //      *
    //    /   \
    //   a     +
    //       /   \
    //      -     d
    //    /   \
    //   b     c
    // is "35" when a is 5, b is 3, c is 2, and d is 6

    let mut env = Environment::new();
    env.set("a".to_string(), AST::Number(5));
    env.set("b".to_string(), AST::Number(3));
    env.set("c".to_string(), AST::Number(2));
    env.set("d".to_string(), AST::Number(6));

    let mut evaluator = Evaluator::new(&mut env);

    let ast = AST::Times(box AST::Variable("a".to_string()),
                         box AST::Plus(box AST::Minus(box AST::Variable("b".to_string()),
                                                      box AST::Variable("c".to_string())),
                                       box AST::Variable("d".to_string())));
    let result = evaluator.evaluate(ast);

    assert_eq!(result, Some(AST::Number(35)));
}

#[test]
fn test_float_ast() {
    // Test that the result for ast:
    //      *
    //    /   \
    //   a     +
    //       /   \
    //      -     c
    //    /   \
    //   1.5   b
    // is "27.5" when a is 5, b is 2, and c is 6

    let mut env = Environment::new();
    env.set("a".to_string(), AST::Number(5));
    env.set("b".to_string(), AST::Number(2));
    env.set("c".to_string(), AST::Number(6));

    let mut evaluator = Evaluator::new(&mut env);

    let ast = AST::Times(box AST::Variable("a".to_string()),
                         box AST::Plus(box AST::Minus(box AST::Decimal(1.5),
                                                      box AST::Variable("b".to_string())),
                                       box AST::Variable("c".to_string())));
    let result = evaluator.evaluate(ast);

    assert_eq!(result, Some(AST::Decimal(27.5)));
}
