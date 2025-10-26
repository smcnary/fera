use crate::ast::{Type, Program, Item, Expr};
use std::collections::HashMap;

pub struct TypeChecker {
    types: HashMap<String, Type>,
    functions: HashMap<String, (Type, Vec<Type>)>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        // First pass: collect type definitions and function signatures
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    self.types.insert(s.name.clone(), Type::Struct(s.name.clone()));
                }
                Item::Union(u) => {
                    self.types.insert(u.name.clone(), Type::Union(u.name.clone()));
                }
                Item::Enum(e) => {
                    self.types.insert(e.name.clone(), Type::Enum(e.name.clone()));
                }
                Item::TypeDef(td) => {
                    self.types.insert(td.name.clone(), td.ty.clone());
                }
                Item::Function(f) => {
                    let param_types = f.params.iter().map(|p| p.ty.clone()).collect();
                    self.functions.insert(f.name.clone(), (f.return_type.clone(), param_types));
                }
                _ => {}
            }
        }
        
        // Second pass: type check function bodies
        for item in &program.items {
            if let Item::Function(f) = item {
                if let Some(body) = &f.body {
                    self.check_block(body)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn check_block(&self, _block: &crate::ast::Block) -> Result<(), String> {
        // TODO: Implement statement type checking
        Ok(())
    }
    
    pub fn infer_type(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::IntLiteral(_, _) => Ok(Type::I32),
            Expr::FloatLiteral(_, _) => Ok(Type::F64),
            Expr::StringLiteral(_, _) => Ok(Type::Pointer(
                Box::new(Type::Char),
                vec![crate::ast::TypeQualifier::Const],
            )),
            Expr::CharLiteral(_, _) => Ok(Type::Char),
            Expr::BoolLiteral(_, _) => Ok(Type::Bool),
            Expr::Identifier(name, _) => {
                self.types.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Unknown identifier: {}", name))
            }
            Expr::Binary(_, left, right, _) => {
                let left_ty = self.infer_type(left)?;
                let right_ty = self.infer_type(right)?;
                // TODO: Proper type coercion rules
                Ok(left_ty)
            }
            Expr::Unary(_, expr, _) => self.infer_type(expr),
            Expr::Call(func, args, _) => {
                if let Expr::Identifier(name, _) = func.as_ref() {
                    if let Some((ret_ty, _)) = self.functions.get(name) {
                        Ok(ret_ty.clone())
                    } else {
                        Err(format!("Unknown function: {}", name))
                    }
                } else {
                    Err("Function call on non-identifier".to_string())
                }
            }
            _ => Err("Type inference not implemented for this expression".to_string()),
        }
    }
}

