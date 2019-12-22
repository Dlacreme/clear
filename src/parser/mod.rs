use pest::Parser;
use crate::ast;
use crate::log;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct SourceFile;

pub fn parse_file(filename: &str) -> Vec<ast::Node> {
    let mut ast: Vec<ast::Node> = vec!();
    let raw_file_content = std::fs::read_to_string(filename).expect(format!("Cannot read file {}", filename).as_str());
    let mut index = 0;
    match SourceFile::parse(Rule::file, &raw_file_content) {
        Ok(mut next) => {
            index += 1;
            let parsed_file_content = next.next().unwrap();

            for pair in parsed_file_content.into_inner() {
                let ast_node = match_rules_to_ast_node(pair, index);
                if ast_node.is_some() {
                    ast.push(ast_node.unwrap());
                }
            }
        },
        Err(_) => {
            crate::log::error(format!("Could not parse {}", filename));
            std::process::exit(-2);
        }
    }
    ast
}

fn match_rules_to_ast_node(pair: pest::iterators::Pair<'_, Rule>, index: i32) -> Option<ast::Node> {
    match pair.as_rule() {
        Rule::libfunc => Some(libfunc(pair.into_inner())),
        _ => None,
    }
}

fn libfunc(pairs: pest::iterators::Pairs<'_, Rule>) -> ast::Node {
    let mut func_name: &str = "";
    let mut params: Vec<String> = vec!();
    for pair in pairs {
        match pair.as_rule() {
            Rule::libfuncnames => {
                func_name = pair.as_span().as_str();
            },
            Rule::string => {
                params.push(String::from(pair.as_span().as_str()));
            }
            _ => ()
        }
    }
    if func_name.is_empty() {
        log::error("Invalid lib function call");
        std::process::exit(-3);
    }
    ast::Node::libfunc(String::from(func_name), params, String::from("dqwdq"), 1, 2)
}
