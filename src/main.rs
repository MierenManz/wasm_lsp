mod wasm_lexer;

use logos::Logos;
use wasm_lexer::Tokens;

fn main() {
    let s = std::fs::read_to_string("./test/named_ptrs.wat").unwrap();
    let start = std::time::Instant::now();
    let lexer = Tokens::lexer(&s);
    let end = start.elapsed().as_nanos();
    for token in lexer {
        println!("{:?}", token)
    }
    println!("{}", end);
}