use super::structs::Function;
use super::structs::ModuleItems;
use super::structs::Type;
use super::Tokens;
use logos::Lexer;

pub struct Module<'a> {
  items: Vec<ModuleItems<'a>>,
}

pub fn parse<'a, 'b>(lexer: &mut Lexer<'a, Tokens<'a>>) -> Result<Module<'b>, String> {
  let mut ast = Module { items: Vec::new() };

  loop {
    let statement = lexer.next();
    if statement.is_none() {
      break;
    }

    match statement.unwrap() {}
  }

  Ok(ast)
}
