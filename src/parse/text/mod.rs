mod token;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/text/parser.rs");

pub use token::Token;
pub use parser::TextMapParser as Parser;
