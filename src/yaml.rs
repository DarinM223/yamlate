use ast::{Exp, Lit};
use environment::Environment;
use errors::YamlError;
use lexer::Lexer;
use linked_hash_map::LinkedHashMap;
use parser::Parser;
use yaml_rust::yaml::Yaml;

#[derive(Debug, PartialEq)]
pub enum YamlType {
    Value(Yaml),
    Return(Yaml),
}

// same as apply_keywords but only works on nested keywords in while statements
fn apply_nested_while_keywords(
    h: &LinkedHashMap<Yaml, Yaml>,
    prop_str: &str,
    env: &mut dyn Environment,
) -> Result<YamlType, YamlError> {
    for (key, val) in h {
        if let Yaml::String(ref keyword) = *key {
            if keyword.as_str() == "do" {
                loop {
                    // check proposition if true
                    let result = evaluate_helper(&Yaml::String(prop_str.to_owned()), env)?;
                    if result == YamlType::Value(Yaml::Boolean(false)) {
                        break;
                    }

                    env.push();

                    // evaluate commands inside do block
                    evaluate_helper(val, env)?;

                    env.pop();
                }
            }
        }
    }

    Ok(YamlType::Value(Yaml::Hash(h.clone())))
}

// same as apply_keywords but only works on nested keywords in if statements
// like do or else
fn apply_nested_if_keywords(
    h: &LinkedHashMap<Yaml, Yaml>,
    prop_str: &str,
    env: &mut dyn Environment,
) -> Result<YamlType, YamlError> {
    for (key, val) in h {
        if let Yaml::String(ref keyword) = *key {
            let result = evaluate_helper(&Yaml::String(prop_str.to_owned()), env)?;

            match keyword.as_str() {
                "do" => {
                    if result == YamlType::Value(Yaml::Boolean(true)) {
                        env.push();
                        let result = evaluate_helper(val, env)?;
                        env.pop();
                        return Ok(result);
                    }
                }
                "else" => {
                    if result == YamlType::Value(Yaml::Boolean(false)) {
                        env.push();
                        let result = evaluate_helper(val, env)?;
                        env.pop();
                        return Ok(result);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(YamlType::Value(Yaml::Hash(h.clone())))
}

// applies the effects of keywords in a YAML hash
fn apply_keyword(
    s: &str,
    _k: &Yaml,
    v: &Yaml,
    env: &mut dyn Environment,
) -> Result<YamlType, YamlError> {
    match s {
        "while" | "if" => {
            if let Yaml::Array(ref arr) = *v {
                let mut prop_str = String::new();
                for val in arr {
                    // Builds main propositional logic by anding the logic statements in the list
                    // together
                    if let Yaml::String(ref s) = *val {
                        if s.as_str().contains("~>") {
                            let split_vec = s.as_str().split("~>").collect::<Vec<_>>();
                            let prop = split_vec[1];

                            let str_len = prop_str.len();
                            if str_len == 0 {
                                prop_str = format!("~> ({})", prop);
                            } else {
                                prop_str = format!("{} && ({})", prop_str, prop);
                            }
                        }
                    } else if let Yaml::Hash(ref h) = *val {
                        // applies logic based on the type of keyword
                        match s {
                            "if" => {
                                return apply_nested_if_keywords(h, prop_str.clone().as_str(), env)
                            }
                            "while" => {
                                return apply_nested_while_keywords(
                                    h,
                                    prop_str.clone().as_str(),
                                    env,
                                )
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        "return" => {
            let result = evaluate_helper(&v, env)?;
            if let YamlType::Value(val) = result {
                return Ok(YamlType::Return(val));
            }
            return Ok(result);
        }
        _ => {}
    }

    Ok(YamlType::Value(v.clone()))
}

// evaluates the result of a fragment of YAML
fn evaluate_helper(yaml: &Yaml, env: &mut dyn Environment) -> Result<YamlType, YamlError> {
    match *yaml {
        Yaml::String(ref s) => {
            if s.as_str().contains("~>") {
                let split_vec = s.as_str().split("~>").collect::<Vec<_>>();
                let mut parser = Parser::new();

                let mut lexer = Lexer::new();
                lexer.parse_string(split_vec[1])?;
                let ast =
                    parser.parse_to_ast(&mut lexer.state.variables, &mut lexer.state.operators)?;
                let result = ast.eval(env)?;

                Ok(YamlType::Value(match result {
                    Exp::Lit(Lit::Decimal(d)) => Yaml::Real(d.to_string()),
                    Exp::Lit(Lit::Number(n)) => Yaml::Integer(n as i64),
                    Exp::Lit(Lit::Bool(b)) => Yaml::Boolean(b),
                    Exp::Lit(Lit::Str(s)) => Yaml::String(s),
                    _ => Yaml::String(split_vec[1].to_owned()),
                }))
            } else {
                Ok(YamlType::Value(Yaml::String(s.clone())))
            }
        }
        Yaml::Array(ref arr) => {
            let mut last_value: Option<Yaml> = None;
            for v in arr {
                let result = evaluate_helper(v, env)?;
                if let YamlType::Return(val) = result {
                    return Ok(YamlType::Return(val));
                } else if let YamlType::Value(val) = result {
                    last_value = Some(val);
                }
            }

            if let Some(val) = last_value {
                Ok(YamlType::Value(val))
            } else {
                Ok(YamlType::Value(Yaml::Array(arr.clone())))
            }
        }
        Yaml::Hash(ref h) => {
            for (k, v) in h {
                if let Yaml::String(ref s) = *k {
                    return apply_keyword(s.as_str(), k, v, env);
                }
            }
            Ok(YamlType::Value(Yaml::Hash(h.clone())))
        }
        ref val @ _ => Ok(YamlType::Value(val.clone())),
    }
}

// Main function for evaluating YAML
pub fn evaluate(yaml: &Yaml, env: &mut dyn Environment) -> Result<Yaml, YamlError> {
    let result = evaluate_helper(yaml, env)?;

    Ok(match result {
        YamlType::Value(v) => v,
        YamlType::Return(v) => v,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Lit;
    use environment::{ASTEnvironment, Environment};
    use yaml_rust::yaml::Yaml;
    use yaml_rust::YamlLoader;

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

        let mut env = ASTEnvironment::new();
        env.set("a", Lit::Number(1));
        env.set("b", Lit::Number(2));

        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(15)));
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
        let mut env = ASTEnvironment::new();
        env.set("a", Lit::Number(1));
        env.set("b", Lit::Number(2));
        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(20)));
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
        let mut env = ASTEnvironment::new();
        env.set("a", Lit::Number(1));
        env.set("b", Lit::Number(2));
        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(10)));
        assert_eq!(env.get("a"), Some(Lit::Number(1)));
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
        let mut env = ASTEnvironment::new();
        env.set("a", Lit::Number(1));
        env.set("b", Lit::Number(2));
        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(10)));
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
        let mut env = ASTEnvironment::new();
        env.set("a", Lit::Number(1));
        env.set("b", Lit::Number(2));
        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(10)));
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
        let mut env = ASTEnvironment::new();
        let docs = YamlLoader::load_from_str(s).unwrap();
        assert_eq!(evaluate(&docs[0]["foo"], &mut env), Ok(Yaml::Integer(5)));
        assert_eq!(env.get("a"), Some(Lit::Number(5)));
    }
}
