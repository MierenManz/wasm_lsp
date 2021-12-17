use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub enum Tokens<'a> {
    #[error]
    Error,

    #[token("(")]
    NewScope,

    #[token(")")]
    ScopeClose,

    #[token("module")]
    Module,

    #[token("func")]
    Func,

    #[token("param")]
    Param,

    #[token("result")]
    Result,

    #[token("local")]
    Local,

    #[token("global")]
    Global,

    #[regex(r"\.[0-9a-z_\.]+")]
    DotExpression(&'a str),

    #[token("i64")]
    #[token("i32")]
    Type(&'a str),

    #[regex("\\$[\x21-\x7E]+")]
    #[regex(r"[0-9]+")]
    Value(&'a str),

    #[regex(r"[\s]+", logos::skip)]
    WhiteSpace,
}
