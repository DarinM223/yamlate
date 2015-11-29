extern crate num;

use ast::AST;
use environment::{IEnvironment, Environment};
use helpers::ast_to_operator;
use self::num::traits::Num;

pub struct Evaluator<'a> {
    env: &'a mut IEnvironment,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut IEnvironment) -> Self {
        Evaluator { env: env }
    }

    fn evaluate_arithmetic(&mut self,
                           operator: &str,
                           child1: AST,
                           child2: AST)
                           -> Result<AST, String> {
        let result1 = self.evaluate(child1);
        let result2 = self.evaluate(child2);

        match result1 {
            Ok(AST::Number(val)) => {
                let param1: i32 = val;

                match result2 {
                    Ok(AST::Decimal(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1 as f64, param2))),
                    Ok(AST::Number(param2)) =>
                        Ok(AST::Number(apply_arithmetic_operator(operator, param1, param2))),
                    _ => Err("Right hand result is not a numeric value".to_string()),
                }
            }
            Ok(AST::Decimal(val)) => {
                let param1: f64 = val;

                match result2 {
                    Ok(AST::Number(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1, param2 as f64))),
                    Ok(AST::Decimal(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1, param2))),
                    _ => Err("Right hand result is not a numeric value".to_string()),
                }
            }
            _ => Err("Left hand result is not a numeric value".to_string()),
        }
    }

    fn evaluate_set_variable(&mut self,
                             operator: &str,
                             child1: AST,
                             child2: AST)
                             -> Result<AST, String> {
        let result = self.evaluate(child2).unwrap_or(AST::None);
        if result == AST::None {
            return Ok(result);
        }

        if let AST::Variable(name) = child1 {
            if operator == ":=" {
                self.env.set(name, result.clone());
            } else if operator == "=" {
                self.env.assign(name, result.clone());
            } else {
                return Err("Variable setting operator not implemented".to_string());
            }

            Ok(result)
        } else {
            Err("Left hand result must be a variable".to_string())
        }
    }

    fn evaluate_equality(&mut self,
                         operator: &str,
                         child1: AST,
                         child2: AST)
                         -> Result<AST, String> {
        let result1 = self.evaluate(child1).unwrap_or(AST::None);
        let result2 = self.evaluate(child2).unwrap_or(AST::None);

        match operator {
            "==" => {
                if result1.eq(&result2) {
                    Ok(AST::Number(1))
                } else {
                    Ok(AST::Number(0))
                }
            }
            "!=" => {
                if !result1.eq(&result2) {
                    Ok(AST::Number(1))
                } else {
                    Ok(AST::Number(0))
                }
            }
            _ => Err("Equality operator not implemented".to_string()),
        }
    }

    fn evaluate_boolean(&mut self,
                        operator: &str,
                        child1: AST,
                        child2: AST)
                        -> Result<AST, String> {
        if let AST::Number(val1) = self.evaluate(child1).unwrap_or(AST::None) {
            if val1 == 0 && operator == "&&" || val1 > 0 && operator == "||" {
                return Ok(AST::Number(val1));
            }

            if let AST::Number(val2) = self.evaluate(child2).unwrap_or(AST::None) {
                Ok(AST::Number(val2))
            } else {
                Err("Right hand result must be a number".to_string())
            }
        } else {
            Err("Left hand result must be a number".to_string())
        }
    }

    /// evaluate evaluates the given AST and returns an AST 
    /// of the result 
    pub fn evaluate(&mut self, ast: AST) -> Result<AST, String> {
        let op = ast_to_operator(&ast);
        match ast {
            AST::Variable(name) => match self.env.get(name) {
                Some(val) => Ok(val.clone()),
                None => Err("Variable is not in environment".to_string()),
            },

            ast @ AST::Number(_) | ast @ AST::String(_) | ast @ AST::Decimal(_) => Ok(ast),

            AST::Assign(box child1, box child2) |
            AST::Declare(box child1, box child2) =>
                self.evaluate_set_variable(op.as_str(), child1, child2),

            AST::Equal(box child1, box child2) |
            AST::NotEqual(box child1, box child2) =>
                self.evaluate_equality(op.as_str(), child1, child2),

            AST::And(box child1, box child2) |
            AST::Or(box child1, box child2) => self.evaluate_boolean(op.as_str(), child1, child2),

            AST::Plus(box child1, box child2) |
            AST::Minus(box child1, box child2) |
            AST::Times(box child1, box child2) |
            AST::Divide(box child1, box child2) |
            AST::Modulo(box child1, box child2) =>
                self.evaluate_arithmetic(op.as_str(), child1, child2),

            _ => Err("Operation is not implemented yet".to_string()),
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

    assert_eq!(result, Ok(AST::Number(35)));
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

    assert_eq!(result, Ok(AST::Number(35)));
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

    assert_eq!(result, Ok(AST::Decimal(27.5)));
}

#[test]
fn test_declare_assign() {
    // Test that evaluating ast:
    //     :=
    //    /  \
    //   x    *
    //       / \
    //      10  +
    //         / \
    //        2  3
    // results in x being bound to 50 in the current scope
    // then after pushing a new scope and evaluating ast:
    //    =
    //   / \
    //  x   +
    //     / \
    //    1   2
    // results in x being set to 3 in the original scope

    let mut env = Environment::new();
    let mut evaluator = Evaluator::new(&mut env);
    let ast = AST::Declare(box AST::Variable("x".to_string()),
                           box AST::Times(box AST::Number(10),
                                          box AST::Plus(box AST::Number(2), box AST::Number(3))));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(50)));
    assert_eq!(evaluator.env.get("x".to_string()), Some(&AST::Number(50)));

    evaluator.env.push();

    let ast = AST::Assign(box AST::Variable("x".to_string()),
                          box AST::Plus(box AST::Number(1), box AST::Number(2)));

    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(3)));

    evaluator.env.pop();
    assert_eq!(evaluator.env.get("x".to_string()), Some(&AST::Number(3)));
}

#[test]
fn test_equality() {
    let mut env = Environment::new();
    let mut evaluator = Evaluator::new(&mut env);

    // Test number equality

    let ast = AST::Equal(box AST::Number(5), box AST::Number(5));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(1)));

    let ast = AST::Equal(box AST::Number(5), box AST::Number(4));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));

    // Test decimal equality

    let ast = AST::Equal(box AST::Decimal(2.56), box AST::Decimal(2.56));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(1)));

    let ast = AST::Equal(box AST::Decimal(2.56), box AST::Decimal(2.55));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));

    // Test string equality

    let ast = AST::Equal(box AST::String("Hello".to_string()),
                         box AST::String("Hello".to_string()));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(1)));

    let ast = AST::Equal(box AST::String("Hello".to_string()),
                         box AST::String("hello".to_string()));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));
}


#[test]
fn test_boolean_operators() {
    let mut env = Environment::new();
    let mut evaluator = Evaluator::new(&mut env);

    // Test and operator

    let ast = AST::And(box AST::Number(1), box AST::Number(0));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));

    let ast = AST::And(box AST::Number(0), box AST::Number(5));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));

    let ast = AST::And(box AST::Number(3), box AST::Number(5));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(5)));

    // Test or operator

    let ast = AST::Or(box AST::Number(5), box AST::Number(1));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(5)));

    let ast = AST::Or(box AST::Number(0), box AST::Number(3));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(3)));

    let ast = AST::Or(box AST::Number(0), box AST::Number(0));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));
}
