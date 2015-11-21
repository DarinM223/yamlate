use ast::AST;
use std::collections::HashMap;

pub trait IEnvironment {
    /// gets a value from the environment
    /// returns the value of the variable in the most
    /// current scope possible or None if the variable 
    /// is not in the scope
    fn get(&self, var: String) -> Option<&AST>;

    /// assign sets an existing value in the environment
    /// difference between set is that set always creates a new binding in 
    /// the current scope whereas assign assigns to the most current scope
    /// of an existing variable (so it can assign to variables in previous scopes if there
    /// is no binding of a variable to the current scope)
    fn assign(&mut self, var: String, value: AST); 

    /// set sets a binding from variable name to a value in the current scope 
    /// of an environment
    fn set(&mut self, var: String, value: AST);

    /// push adds a new scope to the environment
    /// used for blocks like if statements or for loops
    fn push(&mut self);

    /// pop removes a scope from the environment
    /// called after block ends
    fn pop(&mut self);

    /// len returns the number of scopes inside the current scope is
    fn len(&self) -> usize;
}

pub struct Environment {
    stack: Vec<HashMap<String, AST>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { stack: vec![HashMap::new()] }
    }
}

impl IEnvironment for Environment {
    fn get(&self, var: String) -> Option<&AST> {
        for i in (0..self.stack.len()).rev() {
            match self.stack[i].get(&var) {
                val @ Some(_) => return val,
                _ => {}
            }
        }

        None
    }

    fn assign(&mut self, var: String, value: AST) {
        for i in (0..self.stack.len()).rev() {
            if self.stack[i].contains_key(&var) {
                *self.stack[i].get_mut(&var).unwrap() = value;
                return;
            }
        }
    }

    fn set(&mut self, var: String, value: AST) {
        match self.stack.len() {
            0 => {}
            n => {
                self.stack[n - 1].insert(var, value);
            }
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
}

#[test]
fn test_bad_value_empty_stack() {
    let mut env = Environment::new();
    assert_eq!(env.get("Hello".to_string()), None);
}

#[test]
fn test_bad_value_nonempty_stack() {
    let mut env = Environment::new();
    env.set("hello".to_string(), AST::Number(2));
    env.push();
    env.set("world".to_string(), AST::Number(3));
    assert_eq!(env.get("blah".to_string()), None);
}

#[test]
fn test_good_value_one_stack() {
    let mut env = Environment::new();
    env.set("hello".to_string(), AST::Number(2));
    env.set("world".to_string(), AST::Number(3));
    assert_eq!(env.get("world".to_string()), Some(&AST::Number(3)));
}

#[test]
fn test_push_adds_environment() {
    let mut env = Environment::new();
    env.push();
    assert_eq!(env.len(), 2);
}

#[test]
fn test_pop_removes_environment() {
    let mut env = Environment::new();
    env.push();
    env.pop();
    assert_eq!(env.len(), 1);
}

#[test]
fn test_good_value_multiple_stacks() {
    let mut env = Environment::new();
    env.set("hello".to_string(), AST::Number(2));
    env.push();
    env.set("world".to_string(), AST::Number(3));
    assert_eq!(env.get("hello".to_string()), Some(&AST::Number(2)));
}

#[test]
fn test_override_value() {
    let mut env = Environment::new();
    env.set("hello".to_string(), AST::Number(2));
    env.push();
    env.set("hello".to_string(), AST::Number(3));

    assert_eq!(env.get("hello".to_string()), Some(&AST::Number(3)));

    env.pop();
    assert_eq!(env.get("hello".to_string()), Some(&AST::Number(2)));
}

#[test]
fn test_assign_sets_value_in_other_stack() {
    let mut env = Environment::new();
    env.set("hello".to_string(), AST::Number(2));
    env.push();
    env.assign("hello".to_string(), AST::Number(3));
    env.pop();
    
    assert_eq!(env.get("hello".to_string()), Some(&AST::Number(3)));
}
