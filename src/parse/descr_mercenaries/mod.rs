mod lexer;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/descr_mercenaries/parser.rs");

pub use lexer::Token;
pub use parser::PoolsParser as Parser;

#[derive(Debug)]
pub struct Pool {
    pub id: String,
    pub regions: Vec<String>,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Unit {
    pub id: String,
    pub exp: u32,
    pub cost: u32,
    pub replenish: (f64, f64),
    pub max: u32,
    pub initial: u32,
    pub restrict: Vec<String>,
}