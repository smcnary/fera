// High-level IR - desugared AST with name resolution
use crate::ast;

#[derive(Debug, Clone)]
pub struct HirProgram {
    pub items: Vec<HirItem>,
}

#[derive(Debug, Clone)]
pub enum HirItem {
    Function(HirFunction),
    Global(HirGlobal),
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: String,
    pub return_type: ast::Type,
    pub params: Vec<(String, ast::Type)>,
    pub body: HirBlock,
    pub linkage: ast::Linkage,
}

#[derive(Debug, Clone)]
pub struct HirGlobal {
    pub name: String,
    pub ty: ast::Type,
    pub init: Option<HirExpr>,
    pub linkage: ast::Linkage,
}

#[derive(Debug, Clone)]
pub struct HirBlock {
    pub stmts: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let(String, ast::Type, Option<HirExpr>),
    Expr(HirExpr),
    Return(Option<HirExpr>),
    If(HirExpr, Box<HirStmt>, Option<Box<HirStmt>>),
    While(HirExpr, Box<HirStmt>),
    Block(HirBlock),
}

#[derive(Debug, Clone)]
pub enum HirExpr {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
    Binary(ast::BinaryOp, Box<HirExpr>, Box<HirExpr>),
    Unary(ast::UnaryOp, Box<HirExpr>),
    Call(String, Vec<HirExpr>),
    Index(Box<HirExpr>, Box<HirExpr>),
    Field(Box<HirExpr>, String),
    Cast(ast::Type, Box<HirExpr>),
    Assign(Box<HirExpr>, Box<HirExpr>),
}

pub struct HirBuilder {
}

impl HirBuilder {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn lower_program(&mut self, program: &ast::Program) -> Result<HirProgram, String> {
        let mut items = Vec::new();
        
        for item in &program.items {
            match item {
                ast::Item::Function(f) => {
                    if let Some(body) = &f.body {
                        items.push(HirItem::Function(HirFunction {
                            name: f.name.clone(),
                            return_type: f.return_type.clone(),
                            params: f.params.iter()
                                .map(|p| (p.name.clone(), p.ty.clone()))
                                .collect(),
                            body: self.lower_block(body)?,
                            linkage: f.linkage.clone(),
                        }));
                    }
                }
                ast::Item::GlobalVar(g) => {
                    items.push(HirItem::Global(HirGlobal {
                        name: g.name.clone(),
                        ty: g.ty.clone(),
                        init: if let Some(init) = &g.init {
                            Some(self.lower_expr(init)?)
                        } else {
                            None
                        },
                        linkage: g.linkage.clone(),
                    }));
                }
                _ => {}
            }
        }
        
        Ok(HirProgram { items })
    }
    
    fn lower_block(&mut self, block: &ast::Block) -> Result<HirBlock, String> {
        let mut stmts = Vec::new();
        
        for stmt in &block.stmts {
            stmts.push(self.lower_stmt(stmt)?);
        }
        
        Ok(HirBlock { stmts })
    }
    
    fn lower_stmt(&mut self, stmt: &ast::Stmt) -> Result<HirStmt, String> {
        match stmt {
            ast::Stmt::Return(expr, _) => {
                Ok(HirStmt::Return(if let Some(e) = expr {
                    Some(self.lower_expr(e)?)
                } else {
                    None
                }))
            }
            ast::Stmt::Expr(expr) => Ok(HirStmt::Expr(self.lower_expr(expr)?)),
            ast::Stmt::Let(l) => {
                Ok(HirStmt::Let(
                    l.name.clone(),
                    l.ty.clone(),
                    if let Some(init) = &l.init {
                        Some(self.lower_expr(init)?)
                    } else {
                        None
                    },
                ))
            }
            ast::Stmt::If(i) => {
                Ok(HirStmt::If(
                    self.lower_expr(&i.condition)?,
                    Box::new(self.lower_stmt(&i.then_branch)?),
                    if let Some(else_branch) = &i.else_branch {
                        Some(Box::new(self.lower_stmt(else_branch)?))
                    } else {
                        None
                    },
                ))
            }
            ast::Stmt::While(w) => {
                Ok(HirStmt::While(
                    self.lower_expr(&w.condition)?,
                    Box::new(self.lower_stmt(&w.body)?),
                ))
            }
            ast::Stmt::Block(b) => Ok(HirStmt::Block(self.lower_block(b)?)),
            _ => Err("Statement lowering not fully implemented".to_string()),
        }
    }
    
    fn lower_expr(&mut self, expr: &ast::Expr) -> Result<HirExpr, String> {
        match expr {
            ast::Expr::IntLiteral(val, _) => Ok(HirExpr::IntLiteral(*val)),
            ast::Expr::FloatLiteral(val, _) => Ok(HirExpr::FloatLiteral(*val)),
            ast::Expr::StringLiteral(val, _) => Ok(HirExpr::StringLiteral(val.clone())),
            ast::Expr::BoolLiteral(val, _) => Ok(HirExpr::BoolLiteral(*val)),
            ast::Expr::Identifier(name, _) => Ok(HirExpr::Variable(name.clone())),
            ast::Expr::Binary(op, left, right, _) => {
                Ok(HirExpr::Binary(
                    op.clone(),
                    Box::new(self.lower_expr(left)?),
                    Box::new(self.lower_expr(right)?),
                ))
            }
            ast::Expr::Unary(op, expr, _) => {
                Ok(HirExpr::Unary(op.clone(), Box::new(self.lower_expr(expr)?)))
            }
            ast::Expr::Call(func, args, _) => {
                if let ast::Expr::Identifier(name, _) = func.as_ref() {
                    let hir_args = args.iter()
                        .map(|a| self.lower_expr(a))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(HirExpr::Call(name.clone(), hir_args))
                } else {
                    Err("Function call on non-identifier not supported".to_string())
                }
            }
            ast::Expr::Assign(lhs, rhs, _) => {
                Ok(HirExpr::Assign(
                    Box::new(self.lower_expr(lhs)?),
                    Box::new(self.lower_expr(rhs)?),
                ))
            }
            _ => Err("Expression lowering not fully implemented".to_string()),
        }
    }
}

