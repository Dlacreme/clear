use crate::config::Config;
use crate::ast;
use crate::parser;

pub fn build_app(config: Config) {
    crate::log::log(format!("Starting to build {}...", config.app.target));
    let ast: Vec<ast::Node> = vec!();
    read_and_parse(ast, config.app.target.as_str());
}

/**
 * read_and_parse will read our file line by line and build an AST
 */
fn read_and_parse(mut ast: Vec<ast::Node>, filename: &str) -> Vec<ast::Node> {
    ast.append(&mut parser::parse_file(filename));
    ast
}
