use yaml_rust::yaml::Yaml;
use yaml_rust::{YamlLoader, YamlEmitter};
use environment::{IEnvironment, Environment};
use ast::AST;
use evaluator::Evaluator;
use parser::Parser;
use lexer;

pub fn evaluate(yaml: &Yaml, env: &mut IEnvironment) -> Yaml {
    match yaml {
        &Yaml::String(ref s) => {
            if s.as_str().contains("~>") {
                let split_vec = s.as_str().split("~>").collect::<Vec<_>>();
                let mut evaluator = Evaluator::new(env);
                let mut parser = Parser::new();

                let (mut var_deque, mut op_deque) = lexer::parse_string(split_vec[1]).unwrap();
                let ast = parser.parse_to_ast(&mut var_deque, &mut op_deque).unwrap_or(AST::None);
                let result = evaluator.evaluate(ast).unwrap_or(AST::None);

                match result {
                    AST::Decimal(d) => Yaml::Real(d.to_string()),
                    AST::Number(n) => Yaml::Integer(n as i64),
                    AST::String(s) => Yaml::String(s),
                    _ => Yaml::String(split_vec[1].to_string()),
                }
            } else {
                Yaml::String(s.clone())
            }
        }
        &Yaml::Hash(ref h) => Yaml::Hash(h.clone()),
        &ref val @ _ => val.clone(),
    }
}

#[test]
fn test_wrapper() {
    let s = "
    foo: 
      bar:
        return: '~> 2 * (2 + 3)'
    ";
    let mut env = Environment::new();
    env.set("a".to_string(), AST::Number(1));
    env.set("b".to_string(), AST::Number(2));

    let docs = YamlLoader::load_from_str(s).unwrap();

    for doc in &docs {
        assert_eq!(evaluate(&doc["foo"]["bar"]["return"], &mut env), Yaml::Integer(10));
    }
}
