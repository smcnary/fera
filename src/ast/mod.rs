use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    TypeDef(TypeDef),
    GlobalVar(GlobalVar),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub linkage: Linkage,
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Block>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub backing_type: Option<Type>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub ty: Type,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GlobalVar {
    pub linkage: Linkage,
    pub ty: Type,
    pub name: String,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Linkage {
    Export,    // extern
    Internal,  // static
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Bool,
    Char,
    I8, I16, I32, I64, ISize,
    U8, U16, U32, U64, USize,
    F32, F64,
    Pointer(Box<Type>, Vec<TypeQualifier>),
    Array(Box<Type>, Option<usize>),
    Function(Box<Type>, Vec<Type>),
    Struct(String),
    Union(String),
    Enum(String),
    Named(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    Const,
    Volatile,
    Restrict,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Let),
    Return(Option<Expr>, Span),
    If(If),
    While(While),
    For(For),
    DoWhile(DoWhile),
    Switch(Switch),
    Break(Span),
    Continue(Span),
    Block(Block),
    Label(String, Span),
    Goto(String, Span),
    Asm(InlineAsm),
}

#[derive(Debug, Clone)]
pub struct Let {
    pub ty: Type,
    pub name: String,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct For {
    pub init: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DoWhile {
    pub body: Box<Stmt>,
    pub condition: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub value: Expr,
    pub cases: Vec<SwitchCase>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub pattern: Option<Expr>, // None for default
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct InlineAsm {
    pub template: String,
    pub outputs: Vec<String>,
    pub inputs: Vec<String>,
    pub clobbers: Vec<String>,
    pub is_volatile: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntLiteral(i64, Span),
    FloatLiteral(f64, Span),
    StringLiteral(String, Span),
    CharLiteral(char, Span),
    BoolLiteral(bool, Span),
    Identifier(String, Span),
    Binary(BinaryOp, Box<Expr>, Box<Expr>, Span),
    Unary(UnaryOp, Box<Expr>, Span),
    Call(Box<Expr>, Vec<Expr>, Span),
    Index(Box<Expr>, Box<Expr>, Span),
    Field(Box<Expr>, String, Span),
    Cast(Type, Box<Expr>, Span),
    Sizeof(Type, Span),
    Assign(Box<Expr>, Box<Expr>, Span),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>, Span),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr, BitXor, LeftShift, RightShift,
    LogicalAnd, LogicalOr,
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg, Not, BitNot,
    Deref, AddrOf,
    PreInc, PreDec, PostInc, PostDec,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::IntLiteral(_, s) |
            Expr::FloatLiteral(_, s) |
            Expr::StringLiteral(_, s) |
            Expr::CharLiteral(_, s) |
            Expr::BoolLiteral(_, s) |
            Expr::Identifier(_, s) |
            Expr::Binary(_, _, _, s) |
            Expr::Unary(_, _, s) |
            Expr::Call(_, _, s) |
            Expr::Index(_, _, s) |
            Expr::Field(_, _, s) |
            Expr::Cast(_, _, s) |
            Expr::Sizeof(_, s) |
            Expr::Assign(_, _, s) |
            Expr::Ternary(_, _, _, s) => s.clone(),
        }
    }
}

