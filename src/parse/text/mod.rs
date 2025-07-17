mod lexer;
use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/text/parser.rs");

pub use lexer::Token;
pub use parser::TextMapParser as Parser;

#[derive(Debug)]
pub struct TextMap {
    map: HashMap<String, String>,
}

impl TextMap {
    pub fn merge(&mut self, other: TextMap) {
        self.map.extend(other.map)
    }
}
