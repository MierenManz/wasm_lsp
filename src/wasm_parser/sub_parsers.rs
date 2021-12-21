use super::super::structs::Module;
use super::super::structs::ModuleItems;
use super::super::wasm_lexer::Tokens;
use logos::Lexer;

pub fn get_module<'a>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<Module, &'static str> {
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

pub fn get_module_item<'a>(
    lexer: &mut Lexer<'a, Tokens<'a>>,
    token: Tokens<'a>,
) -> Result<ModuleItems, &'static str> {
    let res = match token {
        Tokens::Func => parse_func(&mut lexer)?,
        Tokens::Global => parse_global(&mut lexer)?,
        Tokens::Memory => parse_memory(&mut lexer)?,
        Tokens::Export => parse_export(&mut lexer)?,
        _ => return Err("Unexpected token"),
    };

    Ok(res)
}
