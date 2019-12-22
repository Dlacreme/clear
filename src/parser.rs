use pest::Parser;
use crate::ast;

#[derive(Parser)]
#[grammar = "parser-clear.pest"]
pub struct SourceFile;

pub fn parse_file(filename: &str) -> Vec<ast::Node> {
    let mut ast: Vec<ast::Node> = vec!();
    let raw_file_content = std::fs::read_to_string(filename).expect(format!("Cannot read file {}", filename).as_str());
    let parsed_file_content = SourceFile::parse(Rule::file, &raw_file_content).expect(format!("Failed to parse file {}", filename).as_str())
        .next().unwrap(); // ;doc says it never fails...

    for pair in parsed_file_content.into_inner() {
        let ast_node = match_rules_to_ast_node(pair);
        if ast_node.is_some() {
            ast.push(ast_node.unwrap());
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

