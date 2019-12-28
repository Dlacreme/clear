use crate::config::Config;
use crate::ast;
use crate::parser;
use crate::gen;

pub fn build_app(config: Config) {
    crate::log::log(format!("Starting to build {}...", config.app.target));
    let mut tree: ast::Tree = ast::Tree::new();
    tree = read_and_parse(tree, config.app.target);
    gen::gen_code(tree.nodes);
}

fn read_and_parse(mut tree: ast::Tree, filename: String) -> ast::Tree {
    tree.append_nodes(parser::parse_file(filename));
    tree
}
