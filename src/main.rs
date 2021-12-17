mod structs;
mod wasm_lexer;
mod wasm_parser;
use logos::Logos;
use wasm_lexer::Tokens;

fn main() {
    let s = std::fs::read_to_string("./test/named_ptrs.wat").unwrap();
    let start = std::time::Instant::now();
    let mut lexer = Tokens::lexer(&s);
    let end = start.elapsed().as_nanos();
    wasm_parser::parse(&mut lexer);
}
