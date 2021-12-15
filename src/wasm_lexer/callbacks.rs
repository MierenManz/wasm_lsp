use logos::Lexer;
use super::tokens::Tokens;
use super::types::Type;
type Lex<'a, 'b> = &'a mut Lexer<'b, Tokens>;

#[inline(always)]
pub fn local_ptr(lex: Lex) -> String {
    lex.slice()[10..].to_string()
}
#[inline(always)]
pub fn maybe_ptr(lex: Lex, start: usize) -> Option<String> {
    Some(lex.slice()[start..].to_string())
}

#[inline(always)]
pub fn get_variable_type(lex: Lex, start: usize) -> Type {
    match &lex.slice()[start..] {
        "i32" => Type::I32,
        "i64" => Type::I64,
        _ => unreachable!(),
    }
}

pub fn tuple_ptr_and_type(lex: Lex, skip: usize) -> (Type, Option<String>) {
    let slice = lex.slice();
    let text = &slice[skip..slice.len() - 4];

    (get_variable_type(lex, slice.len() - 4), Some(text.to_string()))
}