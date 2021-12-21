use super::super::wasm_lexer::Tokens;
use logos::Lexer;

pub fn check_error<'a>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<(), String> {
    loop {
        let maybe_statement = lexer.next();
        let span = lexer.span();
        let slice = lexer.slice();
        if let None = maybe_statement {
            break;
        }
        let statement = maybe_statement.unwrap();
        match statement {
            Tokens::Error => {
                let error = format!("Found unexpected token \"{}\" at {}", slice, span.start);
                return Err(error);
            }
            _ => {}
        }
    }

    Ok(())
}
