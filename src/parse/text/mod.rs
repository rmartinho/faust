mod lexer;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser, "/parse/text/parser.rs");

pub use lexer::Token;
pub use parser::TextMapParser as Parser;
