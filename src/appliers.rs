extern crate num;

use ast::AST;
use evaluator::{Evaluator, ASTResult};
use errors::EvalError;
use self::num::traits::Num;
use std::mem;

pub fn apply_arithmetic_operator<T: Num>(operator: &str, a: T, b: T) -> T {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        _ => a + b, 
    }
}

pub trait Applier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult;
}

pub struct ValueApplier {
    value: AST,
}

impl ValueApplier {
    pub fn new(value: AST) -> Self {
        ValueApplier { value: value }
    }
}

impl Applier for ValueApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        Ok(mem::replace(&mut self.value, AST::None))
    }
}

pub struct VariableApplier {
    name: String,
}

impl VariableApplier {
    pub fn new(name: &str) -> Self {
        VariableApplier { name: name.to_owned() }
    }
}

impl Applier for VariableApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        if let Some(val) = evaluator.env.get(self.name.as_str()) {
            Ok(val.clone())
        } else {
            Err(EvalError::new("Variable is not in environment"))
        }
    }
}

pub struct ArithmeticApplier {
    child1: AST,
    child2: AST,
}

impl ArithmeticApplier {
    pub fn new(child1: AST, child2: AST) -> Self {
        ArithmeticApplier {
            child1: child1,
            child2: child2,
        }
    }
}

impl Applier for ArithmeticApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        let result1 = evaluator.evaluate(mem::replace(&mut self.child1, AST::None));
        let result2 = evaluator.evaluate(mem::replace(&mut self.child2, AST::None));

        match result1 {
            Ok(AST::Number(val)) => {
                let param1: i32 = val;

                match result2 {
                    Ok(AST::Decimal(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1 as f64, param2))),
                    Ok(AST::Number(param2)) =>
                        Ok(AST::Number(apply_arithmetic_operator(operator, param1, param2))),
                    _ => Err(EvalError::new("Right hand result is not a numeric value")),
                }
            }
            Ok(AST::Decimal(val)) => {
                let param1: f64 = val;

                match result2 {
                    Ok(AST::Number(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1, param2 as f64))),
                    Ok(AST::Decimal(param2)) =>
                        Ok(AST::Decimal(apply_arithmetic_operator(operator, param1, param2))),
                    _ => Err(EvalError::new("Right hand result is not a numeric value")),
                }
            }
            _ => Err(EvalError::new("Left hand result is not a numeric value")),
        }
    }
}

pub struct AssignmentApplier {
    variable: AST,
    value: AST,
}

impl AssignmentApplier {
    pub fn new(variable: AST, value: AST) -> Self {
        AssignmentApplier {
            variable: variable,
            value: value,
        }
    }
}

impl Applier for AssignmentApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        let value = mem::replace(&mut self.value, AST::None);
        let result = evaluator.evaluate(value).unwrap_or(AST::None);
        if result == AST::None {
            return Ok(result);
        }

        let variable = mem::replace(&mut self.variable, AST::None);
        if let AST::Variable(name) = variable {
            if operator == ":=" {
                evaluator.env.set(name.as_str(), result.clone());
            } else if operator == "=" {
                evaluator.env.assign(name.as_str(), result.clone());
            } else {
                return Err(EvalError::new("Variable setting operator not implemented"));
            }

            Ok(result)
        } else {
            Err(EvalError::new("Left hand result must be a variable"))
        }
    }
}

pub struct EqualityApplier {
    child1: AST,
    child2: AST,
}

impl EqualityApplier {
    pub fn new(child1: AST, child2: AST) -> Self {
        EqualityApplier {
            child1: child1,
            child2: child2,
        }
    }
}

impl Applier for EqualityApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        let child1 = mem::replace(&mut self.child1, AST::None);
        let result1 = evaluator.evaluate(child1).unwrap_or(AST::None);

        let child2 = mem::replace(&mut self.child2, AST::None);
        let result2 = evaluator.evaluate(child2).unwrap_or(AST::None);

        if operator == "==" {
            if result1.eq(&result2) {
                Ok(AST::Number(1))
            } else {
                Ok(AST::Number(0))
            }
        } else if operator == "!=" {
            if !result1.eq(&result2) {
                Ok(AST::Number(1))
            } else {
                Ok(AST::Number(0))
            }
        } else {
            Err(EvalError::new("Equality operator not implemented"))
        }
    }
}

pub struct BooleanApplier {
    child1: AST,
    child2: AST,
}

impl BooleanApplier {
    pub fn new(child1: AST, child2: AST) -> Self {
        BooleanApplier {
            child1: child1,
            child2: child2,
        }
    }
}

impl Applier for BooleanApplier {
    fn evaluate(&mut self, evaluator: &mut Evaluator, operator: &str) -> ASTResult {
        let child1 = mem::replace(&mut self.child1, AST::None);

        if let AST::Number(val1) = evaluator.evaluate(child1).unwrap_or(AST::None) {
            if val1 == 0 && operator == "&&" || val1 > 0 && operator == "||" {
                return Ok(AST::Number(val1));
            }

            let child2 = mem::replace(&mut self.child2, AST::None);
            if let AST::Number(val2) = evaluator.evaluate(child2).unwrap_or(AST::None) {
                Ok(AST::Number(val2))
            } else {
                Err(EvalError::new("Right hand result must be a number"))
            }
        } else {
            Err(EvalError::new("Left hand result must be a number"))
        }
    }
}

pub fn applier_from_ast(ast: AST) -> Result<Box<Applier>, EvalError> {
    match ast {
        AST::Variable(name) => Ok(box VariableApplier::new(name.as_str())),

        value @ AST::Number(_) |
        value @ AST::String(_) |
        value @ AST::Decimal(_) => Ok(box ValueApplier::new(value)),

        AST::Assign(box child1, box child2) |
        AST::Declare(box child1, box child2) => Ok(box AssignmentApplier::new(child1, child2)),

        AST::Equal(box child1, box child2) |
        AST::NotEqual(box child1, box child2) => Ok(box EqualityApplier::new(child1, child2)),

        AST::And(box child1, box child2) |
        AST::Or(box child1, box child2) => Ok(box BooleanApplier::new(child1, child2)),

        AST::Plus(box child1, box child2) |
        AST::Minus(box child1, box child2) |
        AST::Times(box child1, box child2) |
        AST::Divide(box child1, box child2) |
        AST::Modulo(box child1, box child2) => Ok(box ArithmeticApplier::new(child1, child2)),

        _ => Err(EvalError::new("Operation is not implemented yet")),
    }
}
