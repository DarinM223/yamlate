use ast::Op;
use environment::Environment;
use errors::{EvalError, YamlError};
use ast::lit::Lit;

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    /// A unary operator like !
    UnaryOp(Op, Box<Exp>),
    /// A binary operator like + or -
    BinaryOp(Op, Box<Exp>, Box<Exp>),
    /// A variable to retrieve from the environment
    Variable(String),
    /// Bind a variable name to the evaluated expression
    Declare(String, Box<Exp>),
    /// Set an existing variable name to the evaluated expression
    Assign(String, Box<Exp>),
    /// A literal like 2 or "hello"
    Lit(Lit),
}

impl Exp {
    /// Evaluates a expression and returns a Result type wrapping an expression
    pub fn eval(&self, env: &mut Environment) -> Result<Exp, YamlError> {
        match *self {
            Exp::Variable(ref name) => {
                match env.get(name.as_str()) {
                    Some(lit) => Ok(Exp::Lit(lit)),
                    None => Err(YamlError::EvalError(EvalError::VarNotInEnv(name.clone()))),
                }
            }
            Exp::Declare(ref name, ref exp) => {
                if let Exp::Lit(value) = try!(exp.eval(env)) {
                    env.set(name.as_str(), value.clone());
                    Ok(Exp::Lit(value))
                } else {
                    Err(YamlError::EvalError(EvalError::CannotReduceDeclare(exp.clone())))
                }
            }
            Exp::Assign(ref name, ref exp) => {
                if let Exp::Lit(value) = try!(exp.eval(env)) {
                    env.assign(name.as_str(), value.clone());
                    Ok(Exp::Lit(value))
                } else {
                    Err(YamlError::EvalError(EvalError::CannotReduceAssign(exp.clone())))
                }
            }
            Exp::UnaryOp(ref op, ref exp) => {
                if let Exp::Lit(value) = try!(exp.eval(env)) {
                    match *op {
                        Op::Not => Ok(Exp::Lit(try!(!value))),
                        // Non-unary operators (for exhaustiveness checking)
                        Op::Plus | Op::Minus | Op::Times | Op::Divide | Op::Modulo |
                        Op::Exponent | Op::And | Op::Or | Op::Equal | Op::NotEqual => {
                            return Err(YamlError::EvalError(EvalError::NotUnOp(op.clone())))
                        }
                    }
                } else {
                    Err(YamlError::EvalError(EvalError::CannotReduceUnOp(op.clone(), exp.clone())))
                }
            }
            Exp::BinaryOp(ref op, ref exp1, ref exp2) => {
                if let (Exp::Lit(val1), Exp::Lit(val2)) = (try!(exp1.eval(env)),
                                                           try!(exp2.eval(env))) {
                    Ok(Exp::Lit(match *op {
                        Op::Plus => try!(val1 + val2),
                        Op::Minus => try!(val1 - val2),
                        Op::Times => try!(val1 * val2),
                        Op::Divide => try!(val1 / val2),
                        Op::Modulo => try!(val1 % val2),
                        Op::Exponent => try!(val1.exp(val2)),
                        Op::And => try!(val1.and(val2)),
                        Op::Or => try!(val1.or(val2)),
                        Op::Equal => Lit::Bool(val1 == val2),
                        Op::NotEqual => Lit::Bool(val1 != val2),
                        // Non-binary operators (for exhaustiveness checking)
                        Op::Not => {
                            return Err(YamlError::EvalError(EvalError::NotBinOp(op.clone())))
                        }
                    }))
                } else {
                    Err(YamlError::EvalError(EvalError::CannotReduceBinOp(op.clone(),
                                                                          exp1.clone(),
                                                                          exp2.clone())))
                }
            }
            ref lit @ Exp::Lit(_) => Ok(lit.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use environment::{ASTEnvironment, Environment};
    use ast::{Exp, Lit, Op};

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

        let sub_tree = Exp::BinaryOp(Op::Minus,
                                     Box::new(Exp::Lit(Lit::Number(3))),
                                     Box::new(Exp::Lit(Lit::Number(2))));
        let add_tree = Exp::BinaryOp(Op::Plus,
                                     Box::new(sub_tree),
                                     Box::new(Exp::Lit(Lit::Number(6))));
        let times_tree = Exp::BinaryOp(Op::Times,
                                       Box::new(Exp::Lit(Lit::Number(5))),
                                       Box::new(add_tree));

        assert_eq!(times_tree.eval(&mut env), Ok(Exp::Lit(Lit::Number(35))));
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
        env.set("a", Lit::Number(5));
        env.set("b", Lit::Number(3));
        env.set("c", Lit::Number(2));
        env.set("d", Lit::Number(6));

        let (a, b, c, d) = ("a".to_owned(), "b".to_owned(), "c".to_owned(), "d".to_owned());
        let sub_tree = Exp::BinaryOp(Op::Minus,
                                     Box::new(Exp::Variable(b)),
                                     Box::new(Exp::Variable(c)));
        let add_tree = Exp::BinaryOp(Op::Plus, Box::new(sub_tree), Box::new(Exp::Variable(d)));
        let times_tree = Exp::BinaryOp(Op::Times, Box::new(Exp::Variable(a)), Box::new(add_tree));

        assert_eq!(times_tree.eval(&mut env), Ok(Exp::Lit(Lit::Number(35))));
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
        env.set("a", Lit::Number(5));
        env.set("b", Lit::Number(2));
        env.set("c", Lit::Number(6));

        let (a, b, c) = ("a".to_owned(), "b".to_owned(), "c".to_owned());

        let sub_tree = Exp::BinaryOp(Op::Minus,
                                     Box::new(Exp::Lit(Lit::Decimal(1.5))),
                                     Box::new(Exp::Variable(b)));
        let add_tree = Exp::BinaryOp(Op::Plus, Box::new(sub_tree), Box::new(Exp::Variable(c)));
        let times_tree = Exp::BinaryOp(Op::Times, Box::new(Exp::Variable(a)), Box::new(add_tree));

        assert_eq!(times_tree.eval(&mut env), Ok(Exp::Lit(Lit::Decimal(27.5))));
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
        let add_tree = Exp::BinaryOp(Op::Plus,
                                     Box::new(Exp::Lit(Lit::Number(2))),
                                     Box::new(Exp::Lit(Lit::Number(3))));
        let times_tree = Exp::BinaryOp(Op::Times,
                                       Box::new(Exp::Lit(Lit::Number(10))),
                                       Box::new(add_tree));
        let declare_tree = Exp::Declare("x".to_owned(), Box::new(times_tree));

        assert_eq!(declare_tree.eval(&mut env), Ok(Exp::Lit(Lit::Number(50))));
        assert_eq!(env.get("x"), Some(Lit::Number(50)));

        env.push();

        let add_tree = Exp::BinaryOp(Op::Plus,
                                     Box::new(Exp::Lit(Lit::Number(1))),
                                     Box::new(Exp::Lit(Lit::Number(2))));
        let assign_tree = Exp::Assign("x".to_owned(), Box::new(add_tree));

        assert_eq!(assign_tree.eval(&mut env), Ok(Exp::Lit(Lit::Number(3))));

        env.pop();
        assert_eq!(env.get("x"), Some(Lit::Number(3)));
    }

    #[test]
    fn test_equality() {
        let mut env = ASTEnvironment::new();

        // Test number equality

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Number(5))),
                                Box::new(Exp::Lit(Lit::Number(5))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(true))));

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Number(5))),
                                Box::new(Exp::Lit(Lit::Number(4))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(false))));

        // Test decimal equality

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Decimal(2.56))),
                                Box::new(Exp::Lit(Lit::Decimal(2.56))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(true))));

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Decimal(2.56))),
                                Box::new(Exp::Lit(Lit::Decimal(2.55))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(false))));

        // Test string equality

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Str("Hello".to_owned()))),
                                Box::new(Exp::Lit(Lit::Str("Hello".to_owned()))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(true))));

        let ast = Exp::BinaryOp(Op::Equal,
                                Box::new(Exp::Lit(Lit::Str("Hello".to_owned()))),
                                Box::new(Exp::Lit(Lit::Str("hello".to_owned()))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(false))));
    }


    #[test]
    fn test_boolean_operators() {
        let mut env = ASTEnvironment::new();

        // Test and operator

        let ast = Exp::BinaryOp(Op::And,
                                Box::new(Exp::Lit(Lit::Bool(true))),
                                Box::new(Exp::Lit(Lit::Bool(true))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(true))));

        let ast = Exp::BinaryOp(Op::And,
                                Box::new(Exp::Lit(Lit::Bool(true))),
                                Box::new(Exp::Lit(Lit::Bool(false))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(false))));

        // Test or operator

        let ast = Exp::BinaryOp(Op::Or,
                                Box::new(Exp::Lit(Lit::Bool(true))),
                                Box::new(Exp::Lit(Lit::Bool(false))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(true))));

        let ast = Exp::BinaryOp(Op::Or,
                                Box::new(Exp::Lit(Lit::Bool(false))),
                                Box::new(Exp::Lit(Lit::Bool(false))));
        assert_eq!(ast.eval(&mut env), Ok(Exp::Lit(Lit::Bool(false))));
    }
}
