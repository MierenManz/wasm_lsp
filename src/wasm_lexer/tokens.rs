use logos::Logos;
use super::callbacks::local_ptr;
use super::callbacks::get_variable_type;
use super::callbacks::tuple_ptr_and_type;
use super::callbacks::maybe_ptr;

use super::types::Type;
#[derive(Debug, Logos, PartialEq)]
pub enum Tokens {
    #[error]
    Error,

    #[token("(")]
    NewScope,

    #[token(")")]
    ScopeClose,

    #[regex("module", |_| None as Option<String>)]
    #[regex(r"module[ \t\n\f]+\$[\x20-\x7E\x80-\xFF]+", |lex| maybe_ptr(lex, 7))]
    Module(Option<String>),

    #[regex("func", |_| None as Option<String>)]
    #[regex(r"func[ \t\n\f]+\$[\x20-\x7E\x80-\xFF]+", |lex| maybe_ptr(lex, 5))]
    Func(Option<String>),

    #[regex(r"result (i32|i64)", |lex| get_variable_type(lex, 7))]
    Result(Type),

    #[regex(r"param (i32|i64)", |lex|{ (get_variable_type(lex, 6), None as Option<String>)})]
    #[regex(r"param \$[\x20-\x7E\x80-\xFF]+[ \t\n\f]+(i32|i64)", |lex| tuple_ptr_and_type(lex, 6))]
    Param((Type, Option<String>)),

    #[regex(r"local (i32|i64)", |lex|{ (get_variable_type(lex, 6), None as Option<String>)})]
    #[regex(r"local \$[\x20-\x7E\x80-\xFF]+[ \t\n\f]+(i32|i64)", |lex| tuple_ptr_and_type(lex, 6))]
    LocalVariable((Type, Option<String>)),

    #[regex(r"local.get \$[0-9 a-z A-Z]+", local_ptr)]
    #[regex(r"local.get [0-9]+", local_ptr)]
    LocalGet(String),

    #[regex(r"local.set (\$[0-9 a-z A-Z]|[0-9])+", local_ptr)]
    LocalSet(String),

    #[token("i32.mul")]
    I32Mul,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    WhiteSpace
}
