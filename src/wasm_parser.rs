use super::structs::Function;
use super::structs::Module;
use super::structs::ModuleItems;
use super::structs::Type;
use super::Tokens;
use logos::Lexer;

pub struct Ast<'a> {
    inner: Module<'a>,
}

pub fn parse<'a, 'b>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<Ast<'b>, &'a str> {
    let mut ast = Ast {
        inner: get_module(lexer)?,
    };

    loop {
        let statement = match lexer.next() {
            Some(v) => v,
            None => break,
        };
        if statement != Tokens::NewScope {
            return Err("Unexpected token");
        }
    }

    Ok(ast)
}

pub fn get_module<'a, 'b>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<Module<'b>, &'static str> {
    let new_scope = match lexer.next() {
        Some(v) => v,
        None => return Err("Empty file"),
    };

    let module_token = match lexer.next() {
        Some(v) => v,
        None => return Err("No module found"),
    };

    if new_scope != Tokens::NewScope || module_token != Tokens::Module {
        Err("Unexpected Token")
    } else {
        let module = Module {
            items: Vec::new(),
            span: (1, lexer.source().len() - 1),
        };

        Ok(module)
    }
}
