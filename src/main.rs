// mod ast;
mod wasm_lexer;
use logos::Logos;
use wasm_lexer::Tokens;

#[cfg(not(test))]
fn main() {
    let s = std::fs::read_to_string("./test/full.wat").unwrap();
    let start = std::time::Instant::now();
    let mut lexer = Tokens::lexer(&s);
    let end_lex = start.elapsed().as_nanos();
    let mut buff = String::new();
    loop {
        let statement = lexer.next();
        if let None = statement {
            break;
        }
        let slice = lexer.slice();
        buff.push_str(&format!("{:?}\t\t\t", statement.unwrap()));
        if statement.unwrap() == Tokens::Error {
            buff.push_str(&format!("{}", slice));
        }
        buff.push('\n');
    }
    std::fs::write("./out", buff).unwrap();
    // let start2 = std::time::Instant::now();
    // // wasm_parser::parse(&mut lexer).unwrap();
    // let end_parse = start2.elapsed().as_nanos();
    println!("Lexing time: {}", end_lex) //, end_parse)
}

#[cfg(not(not(test)))]
#[test]
fn full() {
    let s = std::fs::read_to_string("./test/full.wat").unwrap();
    let start = std::time::Instant::now();
    let mut lexer = Tokens::lexer(&s);
    let end_lex = start.elapsed().as_nanos();

    loop {
        let statement = lexer.next();
        if let None = statement {
            break;
        }
        if statement.unwrap() == Tokens::Error {
            panic!("FOUND BAD STATEMENT {}", lexer.slice());
        }
    }
    println!("Lexing time: {}", end_lex) //, end_parse)
}
