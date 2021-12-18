use super::structs::Function;
use super::structs::Module;
use super::structs::ModuleItems;
use super::structs::Type;
use super::Tokens;
use logos::Lexer;

type MutLexer<'a> = &'a mut Lexer<'a, Tokens<'a>>;

pub struct Ast {
    inner: Module,
}

pub fn parse<'a>(lexer: MutLexer) -> Result<Ast, &'a str> {
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

        let next_statement = match lexer.next() {
            Some(v) => v,
            None => return Err("Unexpected EOF"),
        };

        match next_statement {
            Tokens::Func => {
                let function = parse_function(&mut lexer)?;
                ast.inner.items.push(ModuleItems::Function(function));
            },
            Tokens::
        }
    }

    Ok(ast)
}

pub fn get_module<'a>(lexer: MutLexer) -> Result<Module, &'static str> {
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
        let module = Module { items: Vec::new() };

        Ok(module)
    }
}

fn parse_function<'a>(lexer: MutLexer) -> Result<Function, &'a str> {
    let func = Function::default();
    loop {
        let statement = match lexer.next() {
            Some(v) => v,
            None => return Err("Unexpected EOF"),
        };

        match statement {
            Tokens::Error => 
        }
    }
    Ok(func)
}
