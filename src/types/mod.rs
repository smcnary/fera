use crate::ast::{Type, Program, Item, Expr, Stmt, Block};
use std::collections::HashMap;

pub struct TypeChecker {
    types: HashMap<String, Type>,
    functions: HashMap<String, (Type, Vec<Type>)>,
    variables: HashMap<String, Type>,
    current_function_return_type: Option<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            types: HashMap::new(),
            functions: HashMap::new(),
            variables: HashMap::new(),
            current_function_return_type: None,
        };
        
        // Register built-in functions
        checker.register_builtins();
        
        checker
    }
    
    fn register_builtins(&mut self) {
        // I/O functions
        self.functions.insert("print".to_string(), (Type::Void, vec![Type::Pointer(Box::new(Type::Char), vec![])]));
        self.functions.insert("println".to_string(), (Type::Void, vec![Type::Pointer(Box::new(Type::Char), vec![])]));
        self.functions.insert("print_i32".to_string(), (Type::Void, vec![Type::I32]));
        self.functions.insert("print_i64".to_string(), (Type::Void, vec![Type::I64]));
        self.functions.insert("print_f32".to_string(), (Type::Void, vec![Type::F32]));
        self.functions.insert("print_f64".to_string(), (Type::Void, vec![Type::F64]));
        self.functions.insert("print_bool".to_string(), (Type::Void, vec![Type::I32]));
        self.functions.insert("println_i32".to_string(), (Type::Void, vec![Type::I32]));
        self.functions.insert("println_i64".to_string(), (Type::Void, vec![Type::I64]));
        self.functions.insert("println_f32".to_string(), (Type::Void, vec![Type::F32]));
        self.functions.insert("println_f64".to_string(), (Type::Void, vec![Type::F64]));
        self.functions.insert("println_bool".to_string(), (Type::Void, vec![Type::I32]));
        
        // Math functions
        self.functions.insert("abs_i32".to_string(), (Type::I32, vec![Type::I32]));
        self.functions.insert("abs_i64".to_string(), (Type::I64, vec![Type::I64]));
        self.functions.insert("min_i32".to_string(), (Type::I32, vec![Type::I32, Type::I32]));
        self.functions.insert("max_i32".to_string(), (Type::I32, vec![Type::I32, Type::I32]));
        self.functions.insert("sqrt_f32".to_string(), (Type::F32, vec![Type::F32]));
        self.functions.insert("sqrt_f64".to_string(), (Type::F64, vec![Type::F64]));
        self.functions.insert("pow_f32".to_string(), (Type::F32, vec![Type::F32, Type::F32]));
        self.functions.insert("pow_f64".to_string(), (Type::F64, vec![Type::F64, Type::F64]));
        self.functions.insert("sin_f32".to_string(), (Type::F32, vec![Type::F32]));
        self.functions.insert("sin_f64".to_string(), (Type::F64, vec![Type::F64]));
        self.functions.insert("cos_f32".to_string(), (Type::F32, vec![Type::F32]));
        self.functions.insert("cos_f64".to_string(), (Type::F64, vec![Type::F64]));
        self.functions.insert("gcd_i32".to_string(), (Type::I32, vec![Type::I32, Type::I32]));
        self.functions.insert("gcd_i64".to_string(), (Type::I64, vec![Type::I64, Type::I64]));
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
                    // Set up function context
                    self.current_function_return_type = Some(f.return_type.clone());
                    self.variables.clear();
                    
                    // Add parameters to variable scope
                    for param in &f.params {
                        self.variables.insert(param.name.clone(), param.ty.clone());
                    }
                    
                    self.check_block(body)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn check_block(&mut self, block: &Block) -> Result<(), String> {
        for stmt in &block.stmts {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(let_stmt) => {
                if let Some(init) = &let_stmt.init {
                    let expr_type = self.infer_type(init)?;
                    if !self.types_compatible(&let_stmt.ty, &expr_type) {
                        return Err(format!(
                            "Type mismatch in variable '{}': expected {:?}, found {:?}",
                            let_stmt.name, let_stmt.ty, expr_type
                        ));
                    }
                }
                self.variables.insert(let_stmt.name.clone(), let_stmt.ty.clone());
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.infer_type(expr)?;
                Ok(())
            }
            Stmt::Return(expr_opt, _) => {
                let return_type = if let Some(expr) = expr_opt {
                    self.infer_type(expr)?
                } else {
                    Type::Void
                };
                
                if let Some(expected) = &self.current_function_return_type {
                    if !self.types_compatible(expected, &return_type) {
                        return Err(format!(
                            "Return type mismatch: expected {:?}, found {:?}",
                            expected, return_type
                        ));
                    }
                }
                Ok(())
            }
            Stmt::If(if_stmt) => {
                let cond_type = self.infer_type(&if_stmt.condition)?;
                if !matches!(cond_type, Type::Bool | Type::I32) {
                    return Err(format!(
                        "If condition must be boolean or integer, found {:?}",
                        cond_type
                    ));
                }
                self.check_stmt(&if_stmt.then_branch)?;
                if let Some(else_branch) = &if_stmt.else_branch {
                    self.check_stmt(else_branch)?;
                }
                Ok(())
            }
            Stmt::While(while_stmt) => {
                let cond_type = self.infer_type(&while_stmt.condition)?;
                if !matches!(cond_type, Type::Bool | Type::I32) {
                    return Err(format!(
                        "While condition must be boolean or integer, found {:?}",
                        cond_type
                    ));
                }
                self.check_stmt(&while_stmt.body)?;
                Ok(())
            }
            Stmt::Block(block) => self.check_block(block),
            _ => Ok(()), // Other statements not fully implemented
        }
    }
    
    fn types_compatible(&self, expected: &Type, found: &Type) -> bool {
        // Simple type compatibility check
        match (expected, found) {
            (Type::I32, Type::I32) => true,
            (Type::I64, Type::I64) => true,
            (Type::F32, Type::F32) => true,
            (Type::F64, Type::F64) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Char, Type::Char) => true,
            (Type::Void, Type::Void) => true,
            (Type::Pointer(a, _), Type::Pointer(b, _)) => self.types_compatible(a, b),
            _ => false,
        }
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
                // First check variables, then types
                if let Some(ty) = self.variables.get(name) {
                    Ok(ty.clone())
                } else if let Some(ty) = self.types.get(name) {
                    Ok(ty.clone())
                } else {
                    Err(format!("Unknown identifier '{}'. Did you forget to declare it?", name))
                }
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
                    if let Some((ret_ty, param_types)) = self.functions.get(name) {
                        // Check argument count
                        if args.len() != param_types.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, but {} were provided",
                                name, param_types.len(), args.len()
                            ));
                        }
                        
                        // Check argument types
                        for (i, (arg, expected_ty)) in args.iter().zip(param_types.iter()).enumerate() {
                            let arg_ty = self.infer_type(arg)?;
                            if !self.types_compatible(expected_ty, &arg_ty) {
                                return Err(format!(
                                    "Type mismatch in argument {} of function '{}': expected {:?}, found {:?}",
                                    i + 1, name, expected_ty, arg_ty
                                ));
                            }
                        }
                        
                        Ok(ret_ty.clone())
                    } else {
                        Err(format!("Unknown function '{}'. Did you forget to define it?", name))
                    }
                } else {
                    Err("Function call on non-identifier not supported".to_string())
                }
            }
            Expr::Assign(lhs, rhs, _) => {
                let rhs_ty = self.infer_type(rhs)?;
                // For now, just return the rhs type
                // TODO: Check that lhs is assignable
                Ok(rhs_ty)
            }
            _ => Err("Type inference not implemented for this expression".to_string()),
        }
    }
}

