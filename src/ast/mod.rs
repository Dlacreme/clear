mod kind;

use kind::Kind;

#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub filenames: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: Kind,
    loc: Location,
}

#[derive(Debug, Clone)]
struct Location {
    line: i32,
    pos: i32,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: vec!(),
            filenames: vec!(),
        }
    }

    pub fn append_nodes(& mut self, mut nodes: Vec<Node>) {
        self.nodes.append(&mut nodes);
    }

}

impl Node {

    pub fn libfunc(name: String, args: Vec<String>, line: i32, pos: i32) -> Self {
        Self {
            kind: Kind::LibFunc(name, args),
            loc: Location {
                line,
                pos,
            }
        }
    }

}
