use ast::AST;
use std::collections::HashMap;

pub trait Environment {
    /// gets a value from the environment
    /// returns the value of the variable in the most
    /// current scope possible or None if the variable 
    /// is not in the scope
    fn get(&self, var: &str) -> Option<&AST>;

    /// assign sets an existing value in the environment
    /// difference between set is that set always creates a new binding in 
    /// the current scope whereas assign assigns to the most current scope
    /// of an existing variable (so it can assign to variables in previous scopes if there
    /// is no binding of a variable to the current scope)
    fn assign(&mut self, var: &str, value: AST);

    /// set sets a binding from variable name to a value in the current scope 
    /// of an environment
    fn set(&mut self, var: &str, value: AST);

    /// push adds a new scope to the environment
    /// used for blocks like if statements or for loops
    fn push(&mut self);

    /// pop removes a scope from the environment
    /// called after block ends
    fn pop(&mut self);

    /// len returns the number of scopes inside the current scope is
    fn len(&self) -> usize;

    /// is_empty is true if there are no more scopes in the environment and false otherwise
    fn is_empty(&self) -> bool;
}

pub struct ASTEnvironment {
    stack: Vec<HashMap<String, AST>>,
}

impl ASTEnvironment {
    pub fn new() -> Self {
        ASTEnvironment { stack: vec![HashMap::new()] }
    }
}

impl Environment for ASTEnvironment {
    fn get(&self, var: &str) -> Option<&AST> {
        let key = var.to_owned();
        for i in (0..self.stack.len()).rev() {
            if let Some(val) = self.stack[i].get(&key) {
                return Some(val);
            }
        }

        None
    }

    fn assign(&mut self, var: &str, value: AST) {
        let key = var.to_owned();
        for i in (0..self.stack.len()).rev() {
            if self.stack[i].contains_key(&key) {
                *self.stack[i].get_mut(&key).unwrap() = value;
                return;
            }
        }
    }

    fn set(&mut self, var: &str, value: AST) {
        let n = self.stack.len();
        if n > 0 {
            self.stack[n - 1].insert(var.to_owned(), value);
        }
    }

    fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use ast::AST;
    use super::{Environment, ASTEnvironment};

    #[test]
    fn test_bad_value_empty_stack() {
        let env = ASTEnvironment::new();
        assert_eq!(env.get("Hello"), None);
    }

    #[test]
    fn test_bad_value_nonempty_stack() {
        let mut env = ASTEnvironment::new();
        env.set("hello", AST::Number(2));
        env.push();
        env.set("world", AST::Number(3));
        assert_eq!(env.get("blah"), None);
    }

    #[test]
    fn test_good_value_one_stack() {
        let mut env = ASTEnvironment::new();
        env.set("hello", AST::Number(2));
        env.set("world", AST::Number(3));
        assert_eq!(env.get("world"), Some(&AST::Number(3)));
    }

    #[test]
    fn test_push_adds_environment() {
        let mut env = ASTEnvironment::new();
        env.push();
        assert_eq!(env.len(), 2);
    }

    #[test]
    fn test_pop_removes_environment() {
        let mut env = ASTEnvironment::new();
        env.push();
        env.pop();
        assert_eq!(env.len(), 1);
    }

    #[test]
    fn test_good_value_multiple_stacks() {
        let mut env = ASTEnvironment::new();
        env.set("hello", AST::Number(2));
        env.push();
        env.set("world", AST::Number(3));
        assert_eq!(env.get("hello"), Some(&AST::Number(2)));
    }

    #[test]
    fn test_override_value() {
        let mut env = ASTEnvironment::new();
        env.set("hello", AST::Number(2));
        env.push();
        env.set("hello", AST::Number(3));

        assert_eq!(env.get("hello"), Some(&AST::Number(3)));

        env.pop();
        assert_eq!(env.get("hello"), Some(&AST::Number(2)));
    }

    #[test]
    fn test_assign_sets_value_in_other_stack() {
        let mut env = ASTEnvironment::new();
        env.set("hello", AST::Number(2));
        env.push();
        env.assign("hello", AST::Number(3));
        env.pop();

        assert_eq!(env.get("hello"), Some(&AST::Number(3)));
    }
}
