mod structs;
mod wasm_lexer;
mod wasm_parser;
use logos::Logos;
use wasm_lexer::Tokens;

fn main() {
    let s = std::fs::read_to_string("./test/named_ptrs.wat").unwrap();
    let start = std::time::Instant::now();
    let mut lexer = Tokens::lexer(&s);
    let end_lex = start.elapsed().as_nanos();
    let start2 = std::time::Instant::now();
    wasm_parser::parse(&mut lexer).unwrap();
    let end_parse = start2.elapsed().as_nanos();
    println!("Lexing time: {} Parsing time: {}", end_lex, end_parse)
}
