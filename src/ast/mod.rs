mod kind;

use kind::Kind;

#[derive(Debug, Clone)]
pub struct Node {
    kind: Kind,
    loc: Location,
}

#[derive(Debug, Clone)]
struct Location {
    filename: String,
    line: i32,
    pos: i32,
}

impl Node {

    pub fn libfunc(name: String, args: Vec<String>, filename: String, line: i32, pos: i32) -> Self {
        Self {
            kind: Kind::LibFunc(name, args),
            loc: Location {
                filename,
                line,
                pos,
            }
        }
    }

}
