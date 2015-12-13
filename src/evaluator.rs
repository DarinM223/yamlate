use ast::AST;
use environment::{Environment, ASTEnvironment};
use errors::EvalError;
use helpers::ast_to_operator;
use appliers::{Applier, VariableApplier, EqualityApplier, ArithmeticApplier,
                        BooleanApplier, applier_from_ast};

pub struct Evaluator<'a> {
    pub env: &'a mut Environment,
}

pub type ASTResult = Result<AST, EvalError>;

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Evaluator { env: env }
    }

    /// evaluate evaluates the given AST and returns an AST 
    /// of the result 
    pub fn evaluate(&mut self, ast: AST) -> ASTResult {
        let op = ast_to_operator(&ast);

        let result = applier_from_ast(ast);

        if let Ok(mut applier) = result {
            applier.evaluate(self, op.as_str())
        } else if let Err(e) = result {
            Err(e)
        } else {
            Err(EvalError::new("Error"))
        }
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

    let mut env = ASTEnvironment::new();
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

    let mut env = ASTEnvironment::new();
    env.set("a", AST::Number(5));
    env.set("b", AST::Number(3));
    env.set("c", AST::Number(2));
    env.set("d", AST::Number(6));

    let mut evaluator = Evaluator::new(&mut env);

    let ast = AST::Times(box AST::Variable("a".to_owned()),
                         box AST::Plus(box AST::Minus(box AST::Variable("b".to_owned()),
                                                      box AST::Variable("c".to_owned())),
                                       box AST::Variable("d".to_owned())));
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

    let mut env = ASTEnvironment::new();
    env.set("a", AST::Number(5));
    env.set("b", AST::Number(2));
    env.set("c", AST::Number(6));

    let mut evaluator = Evaluator::new(&mut env);

    let ast = AST::Times(box AST::Variable("a".to_owned()),
                         box AST::Plus(box AST::Minus(box AST::Decimal(1.5),
                                                      box AST::Variable("b".to_owned())),
                                       box AST::Variable("c".to_owned())));
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

    let mut env = ASTEnvironment::new();
    let mut evaluator = Evaluator::new(&mut env);
    let ast = AST::Declare(box AST::Variable("x".to_owned()),
                           box AST::Times(box AST::Number(10),
                                          box AST::Plus(box AST::Number(2), box AST::Number(3))));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(50)));
    assert_eq!(evaluator.env.get("x"), Some(&AST::Number(50)));

    evaluator.env.push();

    let ast = AST::Assign(box AST::Variable("x".to_owned()),
                          box AST::Plus(box AST::Number(1), box AST::Number(2)));

    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(3)));

    evaluator.env.pop();
    assert_eq!(evaluator.env.get("x"), Some(&AST::Number(3)));
}

#[test]
fn test_equality() {
    let mut env = ASTEnvironment::new();
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

    let ast = AST::Equal(box AST::String("Hello".to_owned()),
                         box AST::String("Hello".to_owned()));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(1)));

    let ast = AST::Equal(box AST::String("Hello".to_owned()),
                         box AST::String("hello".to_owned()));
    let result = evaluator.evaluate(ast);
    assert_eq!(result, Ok(AST::Number(0)));
}


#[test]
fn test_boolean_operators() {
    let mut env = ASTEnvironment::new();
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
