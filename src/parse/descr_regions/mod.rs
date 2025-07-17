use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;

mod lexer;

lalrpop_mod!(parser, "/parse/descr_regions/parser.rs");

pub use lexer::Token;
pub use parser::RegionsParser as Parser;

#[derive(Debug)]
pub struct Region {
    pub id: String,
    pub legion: Option<String>,
    pub city: String,
    pub faction: String,
    pub rebels: String,
    pub color: (u32, u32, u32),
    pub hidden_resources: Vec<String>,
    pub triumph: u32,
    pub farming: u32,
    pub religions: HashMap<String, u32>,
}
