use yaml_rust::yaml::Yaml;
use yaml_rust::{YamlLoader, YamlEmitter};
use environment::{IEnvironment, Environment};
use ast::AST;
use evaluator::Evaluator;
use parser::Parser;
use helpers::is_keyword;
use lexer;

#[derive(Debug, PartialEq)]
pub enum YamlType {
    Value(Yaml),
    Return(Yaml),
}

pub fn evaluate(yaml: &Yaml, env: &mut IEnvironment) -> YamlType {
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
                let result = evaluate(v, env);
                if let YamlType::Return(val) = result {
                    return YamlType::Return(val);
                } else if let YamlType::Value(val) = result {
                    last_value = Some(val);
                }
            }

            if let Some(val) = last_value {
                YamlType::Return(val)
            } else {
                YamlType::Value(Yaml::Array(arr.clone()))
            }
        }
        &Yaml::Hash(ref h) => {
            for (k, v) in h {
                if let &Yaml::String(ref s) = k {
                    match s.as_str() {
                        "return" => {
                            let result = evaluate(&v, env);
                            if let YamlType::Value(val) = result {
                                return YamlType::Return(val);
                            }
                            return result;
                        }
                        _ => {}
                    }
                }
            }
            YamlType::Value(Yaml::Hash(h.clone()))
        }
        &ref val @ _ => YamlType::Value(val.clone()),
    }
}

#[test]
fn test_yaml_eval() {
    // Test if evaluating "foo" returns 10

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
    env.set("a".to_string(), AST::Number(1));
    env.set("b".to_string(), AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    for doc in &docs {
        assert_eq!(evaluate(&doc["foo"], &mut env),
                   YamlType::Return(Yaml::Integer(10)));
    }
}
