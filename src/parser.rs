use pest::Parser;
use crate::ast;

#[derive(Parser)]
#[grammar = "parser-clear.pest"]
pub struct SourceFile;

pub fn parse_file(filename: &str) -> Vec<ast::Node> {
    let mut ast: Vec<ast::Node> = vec!();
    let raw_file_content = std::fs::read_to_string(filename).expect(format!("Cannot read file {}", filename).as_str());
    match SourceFile::parse(Rule::file, &raw_file_content) {
        Ok(mut next) => {
            let parsed_file_content = next.next().unwrap();

            for pair in parsed_file_content.into_inner() {
                let ast_node = match_rules_to_ast_node(pair);
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

fn match_rules_to_ast_node(pair: pest::iterators::Pair<'_, Rule>) -> Option<ast::Node> {
    match pair.as_rule() {
        Rule::func => {
            println!("THIS IS A FUNCTION YES!!");
            None
        },
        _ => None,
    }
}

