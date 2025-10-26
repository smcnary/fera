use logos::Logos;
use std::ops::Range;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Range<usize>,
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    // Keywords
    #[token("export")]
    Export,
    #[token("internal")]
    Internal,
    #[token("const")]
    Const,
    #[token("volatile")]
    Volatile,
    #[token("restrict")]
    Restrict,
    #[token("typedef")]
    Typedef,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("enum")]
    Enum,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("do")]
    Do,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("goto")]
    Goto,
    #[token("asm")]
    Asm,
    #[token("sizeof")]
    Sizeof,
    #[token("alignas")]
    Alignas,
    #[token("true")]
    True,
    #[token("false")]
    False,
    
    // Types
    #[token("void")]
    Void,
    #[token("bool")]
    Bool,
    #[token("char")]
    Char,
    #[token("int")]
    Int,
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("isize")]
    ISize,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("usize")]
    USize,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    
    // Identifiers
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,
    
    // Literals
    #[regex(r"0x[0-9A-Fa-f]+[uUlL]*")]
    HexInteger,
    #[regex(r"0o[0-7]+[uUlL]*")]
    OctalInteger,
    #[regex(r"0b[01]+[uUlL]*")]
    BinaryInteger,
    #[regex(r"[0-9]+[uUlL]*")]
    DecimalInteger,
    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?[fF]?")]
    FloatLiteral,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[regex(r"'([^'\\]|\\.)+'")]
    CharLiteral,
    
    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("^")]
    Caret,
    #[token("~")]
    Tilde,
    #[token("!")]
    Bang,
    #[token("=")]
    Equals,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    StarEquals,
    #[token("/=")]
    SlashEquals,
    #[token("%=")]
    PercentEquals,
    #[token("&=")]
    AmpersandEquals,
    #[token("|=")]
    PipeEquals,
    #[token("^=")]
    CaretEquals,
    #[token("<<=")]
    LeftShiftEquals,
    #[token(">>=")]
    RightShiftEquals,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("->")]
    Arrow,
    
    // Delimiters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("?")]
    Question,
    
    // Comments
    #[regex(r"//[^\n]*")]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    
    // Preprocessor
    #[regex(r"#[^\n]*")]
    PreprocessorDirective,
    
    // Special
    Eof,
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

