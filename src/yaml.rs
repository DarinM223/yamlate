use yaml_rust::yaml::Yaml;
use std::collections::BTreeMap;
use yaml_rust::YamlLoader;
use environment::{IEnvironment, Environment};
use ast::AST;
use evaluator::Evaluator;
use parser::Parser;
use lexer;

#[derive(Debug, PartialEq)]
pub enum YamlType {
    Value(Yaml),
    Return(Yaml),
}

// same as apply_keywords but only works on nested keywords in while statements
fn apply_nested_while_keywords(h: &BTreeMap<Yaml, Yaml>, prop_str: &str, env: &mut IEnvironment) -> YamlType {
    for (key, val) in h {
        if let &Yaml::String(ref keyword) = key {
            match keyword.as_str() {
                "do" => {
                    loop {
                        // check proposition if true
                        let result = evaluate_helper(&Yaml::String(prop_str.to_string()), env);
                        if let YamlType::Value(Yaml::Integer(i)) = result {
                            if i <= 0 {
                                break;
                            }
                        }

                        env.push();

                        // evaluate commands inside do block
                        evaluate_helper(val, env);

                        env.pop();
                    }
                }
                _ => {}
            }
        }
    }

    YamlType::Value(Yaml::Hash(h.clone()))
}

// same as apply_keywords but only works on nested keywords in if statements
// like do or else
fn apply_nested_if_keywords(h: &BTreeMap<Yaml, Yaml>, prop_str: &str, env: &mut IEnvironment) -> YamlType {
    for (key, val) in h {
        if let &Yaml::String(ref keyword) = key {
            let result = evaluate_helper(&Yaml::String(prop_str.to_string()), env);

            match keyword.as_str() {
                "do" => {
                    if let YamlType::Value(Yaml::Integer(i)) = result {
                        if i > 0 {
                            env.push();
                            let result = evaluate_helper(val, env);
                            env.pop();
                            return result;
                        }
                    }
                }
                "else" => {
                    if let YamlType::Value(Yaml::Integer(i)) = result {
                        if i == 0 {
                            env.push();
                            let result = evaluate_helper(val, env);
                            env.pop();
                            return result;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    YamlType::Value(Yaml::Hash(h.clone()))
}

// applies the effects of keywords in a YAML hash
fn apply_keyword(s: &str, k: &Yaml, v: &Yaml, env: &mut IEnvironment) -> YamlType {
    match s {
        "while" | "if" => {
            if let &Yaml::Array(ref arr) = v {
                let mut prop_str = String::new();
                for val in arr {
                    // Builds main propositional logic by anding the logic statements in the list
                    // together
                    if let &Yaml::String(ref s) = val {
                        if s.as_str().contains("~>") {
                            let split_vec = s.as_str().split("~>").collect::<Vec<_>>();
                            let prop = split_vec[1];

                            let str_len = prop_str.len();
                            if str_len == 0 {
                                prop_str = format!("~> ({})", prop.clone());
                            } else {
                                prop_str = format!("{} && ({})", prop_str, prop.clone());
                            }
                        }
                    } else if let &Yaml::Hash(ref h) = val {
                        // applies logic based on the type of keyword
                        match s {
                            "if" => return apply_nested_if_keywords(h, prop_str.clone().as_str(), env),
                            "while" => return apply_nested_while_keywords(h, prop_str.clone().as_str(), env),
                            _ => {}
                        }
                    }
                }
            }
        }
        "return" => {
            let result = evaluate_helper(&v, env);
            if let YamlType::Value(val) = result {
                return YamlType::Return(val);
            }
            return result;
        }
        _ => {}
    }

    YamlType::Value(v.clone())
}

// evaluates the result of a fragment of YAML
fn evaluate_helper(yaml: &Yaml, env: &mut IEnvironment) -> YamlType {
    match yaml {
        &Yaml::String(ref s) => {
            if s.as_str().contains("~>") {
                let split_vec = s.as_str().split("~>").collect::<Vec<_>>();
                let mut evaluator = Evaluator::new(env);
                let mut parser = Parser::new();

                let (mut var_deque, mut op_deque) = lexer::parse_string(split_vec[1]).unwrap();
                let ast = parser.parse_to_ast(&mut var_deque, &mut op_deque).unwrap_or(AST::None);
                let result = evaluator.evaluate(ast).unwrap_or(AST::None);

                YamlType::Value(match result {
                    AST::Decimal(d) => Yaml::Real(d.to_string()),
                    AST::Number(n) => Yaml::Integer(n as i64),
                    AST::String(s) => Yaml::String(s),
                    _ => Yaml::String(split_vec[1].to_string()),
                })
            } else {
                YamlType::Value(Yaml::String(s.clone()))
            }
        }
        &Yaml::Array(ref arr) => {
            let mut last_value: Option<Yaml> = None;
            for v in arr {
                let result = evaluate_helper(v, env);
                if let YamlType::Return(val) = result {
                    return YamlType::Return(val);
                } else if let YamlType::Value(val) = result {
                    last_value = Some(val);
                }
            }

            if let Some(val) = last_value {
                YamlType::Value(val)
            } else {
                YamlType::Value(Yaml::Array(arr.clone()))
            }
        }
        &Yaml::Hash(ref h) => {
            for (k, v) in h {
                if let &Yaml::String(ref s) = k {
                    return apply_keyword(s.as_str(), k, v, env);
                }
            }
            YamlType::Value(Yaml::Hash(h.clone()))
        }
        &ref val @ _ => YamlType::Value(val.clone()),
    }
}

// Main function for evaluating YAML
pub fn evaluate(yaml: &Yaml, env: &mut IEnvironment) -> Yaml {
    let result = evaluate_helper(yaml, env);

    match result {
        YamlType::Value(v) => v,
        YamlType::Return(v) => v,
    }
}

#[test]
fn test_yaml_eval() {
    // Test if evaluating "foo" returns 15

    let s = "
    foo: 
      - '~> a := 2'
      - if: 
        - '~> a == 2'
        - do:
          - '~> a = 3'
      - return: '~> a * (2 + 3)'
    ";

    let mut env = Environment::new();
    env.set("a", AST::Number(1));
    env.set("b", AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(15));
}

#[test]
fn test_yaml_else() {
    // Test if evaluating "foo" returns 20

    let s = "
    foo: 
      - '~> a := 2'
      - if: 
        - '~> a == 3'
        - do:
            - '~> a = 3'
          else:
            - '~> a = 4'
      - return: '~> a * (2 + 3)' 
    ";

    let mut env = Environment::new();
    env.set("a", AST::Number(1));
    env.set("b", AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(20));
}

#[test]
fn test_return() {
    // Test that return doesn't execute statements after it

    let s = "
    foo: 
      - return: '~> 2 * (2 + 3)'
      - '~> a := 2'
      - if: 
        - '~> a == 2'
        - do:
          - '~> a = 3'
    ";

    let mut env = Environment::new();
    env.set("a", AST::Number(1));
    env.set("b", AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(10));

    assert_eq!(env.get("a"), Some(&AST::Number(1)));
}

#[test]
fn test_return_last_val() {
    // Test that the last value is returned as value instead of return

    let s = "
    foo: 
      - '~> a := 2'
      - if: 
        - '~> a == 2'
        - do:
          - '~> a = 3'
      - '~> 2 * (2 + 3)'
    ";

    let mut env = Environment::new();
    env.set("a", AST::Number(1));
    env.set("b", AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(10));
}

#[test]
fn test_local_variable() {
    // Test that local variable is destroyed after if

    let s = "
    foo: 
      - '~> a := 2'
      - if: 
        - '~> a == 2'
        - do:
          - '~> c := 2'
          - '~> a := 3'
      - '~> a * (2 + 3)'
    ";

    let mut env = Environment::new();
    env.set("a", AST::Number(1));
    env.set("b", AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(10));

    assert_eq!(env.get("c"), None);
}

#[test]
fn test_while_loop() {
    let s = "
    foo:
      - '~> a := 0'
      - while:
        - '~> a != 5'
        - do:
          - '~> a = a + 1'
      - '~> a'
    ";

    let mut env = Environment::new();

    let docs = YamlLoader::load_from_str(s).unwrap();

    assert_eq!(evaluate(&docs[0]["foo"], &mut env), Yaml::Integer(5));

    assert_eq!(env.get("a"), Some(&AST::Number(5)));
}
