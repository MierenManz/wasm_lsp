use super::super::structs::Module;
use super::super::wasm_lexer::Tokens;
use super::sub_parsers::get_module;
use super::sub_parsers::get_module_item;
use super::util::check_error;
use logos::Lexer;

pub struct Ast {
    inner: Module,
}

pub fn parse<'a>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<Ast, String> {
    check_error(&mut lexer.clone())?;
    let mut ast = Ast {
        inner: get_module(lexer)?,
    };

    loop {
        let token = lexer.next();
        let slice = lexer.slice();
        let span = lexer.span();

        let statement = match token {
            Some(v) => v,
            None => break,
        };

        if statement != Tokens::NewScope {
            return Err("Unexpected token".to_string());
        }

        let next_statement = match lexer.next() {
            Some(v) => v,
            None => return Err("Unexpected EOF".to_string()),
        };

        match next_statement {
            Tokens::NewScope => return Err("Unexpected token".to_string()),
            _ => ast
                .inner
                .add_item(get_module_item(&mut lexer, next_statement)?),
        }
    }

    Ok(ast)
}
