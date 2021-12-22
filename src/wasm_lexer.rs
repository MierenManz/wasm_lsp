use logos::Logos;

#[derive(Clone, Copy, Debug, Logos, PartialEq)]
pub enum Tokens<'a> {
    #[error]
    Error,

    #[token("(")]
    NewScope,

    #[token(")")]
    ScopeClose,

    #[token("module")]
    Module,

    #[token("table")]
    Table,

    #[token("func")]
    Func,

    #[token("elem")]
    Element,

    #[token("export")]
    Export,

    #[token("import")]
    Import,

    #[token("memory")]
    Memory,

    #[token("data")]
    Data,

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

    #[token("mut")]
    Mutate,

    #[token("funcref")]
    FuncRef,

    #[token("f64")]
    #[token("f32")]
    Float(&'a str),

    #[token("i64")]
    #[token("i32")]
    Integer(&'a str),

    #[regex("\\$[\x30-\x39\x41-\x5A\x61-\x7A_]+")]
    Reference(&'a str),

    #[regex(r"[0-9]+")]
    Ptr(&'a str),

    #[regex("\"[^\"]+\"")]
    StringLiteral(&'a str),

    #[regex(r"[\s]+", logos::skip)]
    WhiteSpace,

    #[token("type")]
    Type,

    #[token("return")]
    Return,

    #[token("call")]
    Call,

    #[token("call_indirect")]
    IndirectCall,

    #[token("return_call")]
    TailCall,

    #[token("return_call_indirect")]
    IndirectTailCall,
}
