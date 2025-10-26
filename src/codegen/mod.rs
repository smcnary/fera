use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, PointerValue, BasicValueEnum, BasicMetadataValueEnum, BasicValue};
use inkwell::types::{BasicTypeEnum, BasicMetadataTypeEnum, BasicType};
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;

use crate::hir::*;
use crate::ast::{Type, BinaryOp, UnaryOp, Linkage};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        let mut codegen = Self {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_function: None,
        };
        
        // Declare built-in functions
        codegen.declare_builtins();
        
        codegen
    }
    
    fn declare_builtins(&mut self) {
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        let f32_type = self.context.f32_type();
        let f64_type = self.context.f64_type();
        let void_type = self.context.void_type();
        let char_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let void_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        // I/O - String output
        self.module.add_function("print", void_type.fn_type(&[char_ptr_type.into()], false), None);
        self.module.add_function("println", void_type.fn_type(&[char_ptr_type.into()], false), None);
        
        // I/O - Typed output
        self.module.add_function("print_i32", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("print_i64", void_type.fn_type(&[i64_type.into()], false), None);
        self.module.add_function("print_u32", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("print_u64", void_type.fn_type(&[i64_type.into()], false), None);
        self.module.add_function("print_f32", void_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("print_f64", void_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("print_bool", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("print_ptr", void_type.fn_type(&[void_ptr_type.into()], false), None);
        
        self.module.add_function("println_i32", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("println_i64", void_type.fn_type(&[i64_type.into()], false), None);
        self.module.add_function("println_u32", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("println_u64", void_type.fn_type(&[i64_type.into()], false), None);
        self.module.add_function("println_f32", void_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("println_f64", void_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("println_bool", void_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("println_ptr", void_type.fn_type(&[void_ptr_type.into()], false), None);
        
        // Math - Integer functions
        self.module.add_function("abs_i32", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("abs_i64", i64_type.fn_type(&[i64_type.into()], false), None);
        self.module.add_function("min_i32", i32_type.fn_type(&[i32_type.into(), i32_type.into()], false), None);
        self.module.add_function("max_i32", i32_type.fn_type(&[i32_type.into(), i32_type.into()], false), None);
        self.module.add_function("min_i64", i64_type.fn_type(&[i64_type.into(), i64_type.into()], false), None);
        self.module.add_function("max_i64", i64_type.fn_type(&[i64_type.into(), i64_type.into()], false), None);
        self.module.add_function("clamp_i32", i32_type.fn_type(&[i32_type.into(), i32_type.into(), i32_type.into()], false), None);
        self.module.add_function("gcd_i32", i32_type.fn_type(&[i32_type.into(), i32_type.into()], false), None);
        self.module.add_function("gcd_i64", i64_type.fn_type(&[i64_type.into(), i64_type.into()], false), None);
        self.module.add_function("lcm_i32", i32_type.fn_type(&[i32_type.into(), i32_type.into()], false), None);
        self.module.add_function("lcm_i64", i64_type.fn_type(&[i64_type.into(), i64_type.into()], false), None);
        
        // Math - Floating point functions
        self.module.add_function("sqrt_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("sqrt_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("pow_f32", f32_type.fn_type(&[f32_type.into(), f32_type.into()], false), None);
        self.module.add_function("pow_f64", f64_type.fn_type(&[f64_type.into(), f64_type.into()], false), None);
        self.module.add_function("sin_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("sin_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("cos_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("cos_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("tan_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("tan_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("log_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("log_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("exp_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("exp_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("floor_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("floor_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("ceil_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("ceil_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("round_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("round_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        self.module.add_function("abs_f32", f32_type.fn_type(&[f32_type.into()], false), None);
        self.module.add_function("abs_f64", f64_type.fn_type(&[f64_type.into()], false), None);
        
        // String functions
        self.module.add_function("str_len", i64_type.fn_type(&[char_ptr_type.into()], false), None);
        self.module.add_function("str_cmp", i32_type.fn_type(&[char_ptr_type.into(), char_ptr_type.into()], false), None);
        self.module.add_function("str_chr", char_ptr_type.fn_type(&[char_ptr_type.into(), i32_type.into()], false), None);
        self.module.add_function("str_str", char_ptr_type.fn_type(&[char_ptr_type.into(), char_ptr_type.into()], false), None);
        self.module.add_function("str_to_i32", i32_type.fn_type(&[char_ptr_type.into()], false), None);
        self.module.add_function("str_to_i64", i64_type.fn_type(&[char_ptr_type.into()], false), None);
        
        // Character classification
        self.module.add_function("is_digit", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("is_alpha", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("is_alnum", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("is_space", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("to_upper", i32_type.fn_type(&[i32_type.into()], false), None);
        self.module.add_function("to_lower", i32_type.fn_type(&[i32_type.into()], false), None);
        
        // Panic
        self.module.add_function("panic", void_type.fn_type(&[char_ptr_type.into()], false), None);
    }
    
    pub fn codegen_program(&mut self, program: &HirProgram) -> Result<(), String> {
        for item in &program.items {
            match item {
                HirItem::Function(f) => self.codegen_function(f)?,
                HirItem::Global(g) => self.codegen_global(g)?,
            }
        }
        Ok(())
    }
    
    fn codegen_function(&mut self, func: &HirFunction) -> Result<(), String> {
        let ret_type = self.llvm_type(&func.return_type)?;
        
        let param_types: Vec<BasicMetadataTypeEnum> = func.params
            .iter()
            .map(|(_, ty)| {
                self.llvm_type(ty)?
                    .ok_or_else(|| "Function parameters cannot be void".to_string())
                    .map(|t| t.into())
            })
            .collect::<Result<Vec<_>, _>>()?;
        
        let fn_type = match ret_type {
            Some(ty) => ty.fn_type(&param_types, false),
            None => self.context.void_type().fn_type(&param_types, false),
        };
        
        let function = self.module.add_function(&func.name, fn_type, None);
        
        // Set linkage
        match func.linkage {
            Linkage::Export => {
                function.set_linkage(inkwell::module::Linkage::External);
            }
            Linkage::Internal => {
                function.set_linkage(inkwell::module::Linkage::Internal);
            }
        }
        
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        
        self.current_function = Some(function);
        self.variables.clear();
        
        // Allocate and store parameters
        for (i, (name, ty)) in func.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32)
                .ok_or("Parameter index out of range")?;
            
            let llvm_ty = self.llvm_type(ty)?
                .ok_or("Cannot create pointer to void type")?;
            let alloca = self.builder.build_alloca(llvm_ty, name)
                .map_err(|e| format!("Failed to build alloca: {:?}", e))?;
            
            self.builder.build_store(alloca, param_value)
                .map_err(|e| format!("Failed to store parameter: {:?}", e))?;
            
            self.variables.insert(name.clone(), (alloca, llvm_ty));
        }
        
        // Generate function body
        self.codegen_block(&func.body)?;
        
        // Add implicit return if missing
        if let Some(block) = function.get_last_basic_block() {
            if block.get_terminator().is_none() {
                if func.return_type == Type::Void {
                    self.builder.build_return(None)
                        .map_err(|e| format!("Failed to build return: {:?}", e))?;
                } else {
                    // Return zero/null as default
                    let default_val = match ret_type {
                        Some(BasicTypeEnum::IntType(int_ty)) => {
                            int_ty.const_zero().as_basic_value_enum()
                        }
                        Some(BasicTypeEnum::FloatType(float_ty)) => {
                            float_ty.const_zero().as_basic_value_enum()
                        }
                        Some(BasicTypeEnum::PointerType(ptr_ty)) => {
                            ptr_ty.const_null().as_basic_value_enum()
                        }
                        _ => return Err("Cannot create default return value".to_string()),
                    };
                    self.builder.build_return(Some(&default_val))
                        .map_err(|e| format!("Failed to build return: {:?}", e))?;
                }
            }
        }
        
        Ok(())
    }
    
    fn codegen_global(&mut self, global: &HirGlobal) -> Result<(), String> {
        let llvm_ty = self.llvm_type(&global.ty)?
            .ok_or("Cannot create global of void type")?;
        
        let global_var = self.module.add_global(llvm_ty, None, &global.name);
        
        match global.linkage {
            Linkage::Export => {
                global_var.set_linkage(inkwell::module::Linkage::External);
            }
            Linkage::Internal => {
                global_var.set_linkage(inkwell::module::Linkage::Internal);
            }
        }
        
        if let Some(init) = &global.init {
            let init_val = self.codegen_expr(init)?;
            if let BasicValueEnum::IntValue(int_val) = init_val {
                global_var.set_initializer(&int_val);
            }
        }
        
        Ok(())
    }
    
    fn codegen_block(&mut self, block: &HirBlock) -> Result<(), String> {
        for stmt in &block.stmts {
            self.codegen_stmt(stmt)?;
        }
        Ok(())
    }
    
    fn codegen_stmt(&mut self, stmt: &HirStmt) -> Result<(), String> {
        match stmt {
            HirStmt::Let(name, ty, init) => {
                let llvm_ty = self.llvm_type(ty)?
                    .ok_or("Cannot create variable of void type")?;
                
                let alloca = self.builder.build_alloca(llvm_ty, name)
                    .map_err(|e| format!("Failed to build alloca: {:?}", e))?;
                
                if let Some(init_expr) = init {
                    let init_val = self.codegen_expr(init_expr)?;
                    self.builder.build_store(alloca, init_val)
                        .map_err(|e| format!("Failed to store: {:?}", e))?;
                }
                
                self.variables.insert(name.clone(), (alloca, llvm_ty));
                Ok(())
            }
            HirStmt::Expr(expr) => {
                self.codegen_expr(expr)?;
                Ok(())
            }
            HirStmt::Return(expr) => {
                if let Some(ret_expr) = expr {
                    let ret_val = self.codegen_expr(ret_expr)?;
                    self.builder.build_return(Some(&ret_val))
                        .map_err(|e| format!("Failed to build return: {:?}", e))?;
                } else {
                    self.builder.build_return(None)
                        .map_err(|e| format!("Failed to build return: {:?}", e))?;
                }
                Ok(())
            }
            HirStmt::If(cond, then_stmt, else_stmt) => {
                let func = self.current_function.ok_or("No current function")?;
                
                let then_block = self.context.append_basic_block(func, "then");
                let else_block = self.context.append_basic_block(func, "else");
                let merge_block = self.context.append_basic_block(func, "ifcont");
                
                let cond_val = self.codegen_expr(cond)?;
                let cond_int = if let BasicValueEnum::IntValue(iv) = cond_val {
                    iv
                } else {
                    return Err("Condition must be an integer".to_string());
                };
                
                self.builder.build_conditional_branch(cond_int, then_block, else_block)
                    .map_err(|e| format!("Failed to build conditional branch: {:?}", e))?;
                
                // Then block
                self.builder.position_at_end(then_block);
                self.codegen_stmt(then_stmt)?;
                if then_block.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(merge_block)
                        .map_err(|e| format!("Failed to build branch: {:?}", e))?;
                }
                
                // Else block
                self.builder.position_at_end(else_block);
                if let Some(else_s) = else_stmt {
                    self.codegen_stmt(else_s)?;
                }
                if else_block.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(merge_block)
                        .map_err(|e| format!("Failed to build branch: {:?}", e))?;
                }
                
                self.builder.position_at_end(merge_block);
                Ok(())
            }
            HirStmt::While(cond, body) => {
                let func = self.current_function.ok_or("No current function")?;
                
                let cond_block = self.context.append_basic_block(func, "while.cond");
                let body_block = self.context.append_basic_block(func, "while.body");
                let end_block = self.context.append_basic_block(func, "while.end");
                
                self.builder.build_unconditional_branch(cond_block)
                    .map_err(|e| format!("Failed to build branch: {:?}", e))?;
                
                self.builder.position_at_end(cond_block);
                let cond_val = self.codegen_expr(cond)?;
                let cond_int = if let BasicValueEnum::IntValue(iv) = cond_val {
                    iv
                } else {
                    return Err("Condition must be an integer".to_string());
                };
                
                self.builder.build_conditional_branch(cond_int, body_block, end_block)
                    .map_err(|e| format!("Failed to build conditional branch: {:?}", e))?;
                
                self.builder.position_at_end(body_block);
                self.codegen_stmt(body)?;
                if body_block.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_block)
                        .map_err(|e| format!("Failed to build branch: {:?}", e))?;
                }
                
                self.builder.position_at_end(end_block);
                Ok(())
            }
            HirStmt::Block(block) => self.codegen_block(block),
        }
    }
    
    fn codegen_expr(&mut self, expr: &HirExpr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            HirExpr::IntLiteral(val) => {
                Ok(self.context.i32_type().const_int(*val as u64, true).as_basic_value_enum())
            }
            HirExpr::FloatLiteral(val) => {
                Ok(self.context.f64_type().const_float(*val).as_basic_value_enum())
            }
            HirExpr::StringLiteral(s) => {
                let string_val = self.context.const_string(s.as_bytes(), true);
                let global = self.module.add_global(string_val.get_type(), None, ".str");
                global.set_initializer(&string_val);
                global.set_constant(true);
                global.set_linkage(inkwell::module::Linkage::Private);
                
                Ok(global.as_pointer_value().as_basic_value_enum())
            }
            HirExpr::BoolLiteral(val) => {
                Ok(self.context.bool_type().const_int(*val as u64, false).as_basic_value_enum())
            }
            HirExpr::Variable(name) => {
                let (ptr, ty) = self.variables.get(name)
                    .ok_or_else(|| format!("Unknown variable: {}", name))?;
                
                self.builder.build_load(*ty, *ptr, name)
                    .map_err(|e| format!("Failed to load variable: {:?}", e))
            }
            HirExpr::Binary(op, left, right) => {
                let lhs = self.codegen_expr(left)?;
                let rhs = self.codegen_expr(right)?;
                
                match (lhs, rhs) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        let result = match op {
                            BinaryOp::Add => self.builder.build_int_add(l, r, "add"),
                            BinaryOp::Sub => self.builder.build_int_sub(l, r, "sub"),
                            BinaryOp::Mul => self.builder.build_int_mul(l, r, "mul"),
                            BinaryOp::Div => self.builder.build_int_signed_div(l, r, "div"),
                            BinaryOp::Mod => self.builder.build_int_signed_rem(l, r, "mod"),
                            BinaryOp::Equal => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                            BinaryOp::NotEqual => self.builder.build_int_compare(IntPredicate::NE, l, r, "ne"),
                            BinaryOp::Less => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                            BinaryOp::Greater => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                            BinaryOp::LessEqual => self.builder.build_int_compare(IntPredicate::SLE, l, r, "le"),
                            BinaryOp::GreaterEqual => self.builder.build_int_compare(IntPredicate::SGE, l, r, "ge"),
                            BinaryOp::BitAnd => self.builder.build_and(l, r, "and"),
                            BinaryOp::BitOr => self.builder.build_or(l, r, "or"),
                            BinaryOp::BitXor => self.builder.build_xor(l, r, "xor"),
                            BinaryOp::LeftShift => self.builder.build_left_shift(l, r, "shl"),
                            BinaryOp::RightShift => self.builder.build_right_shift(l, r, true, "shr"),
                            _ => return Err(format!("Unsupported binary operator: {:?}", op)),
                        };
                        result.map(|v| v.as_basic_value_enum())
                            .map_err(|e| format!("Failed to build binary op: {:?}", e))
                    }
                    (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                        match op {
                            BinaryOp::Add => self.builder.build_float_add(l, r, "fadd")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float op: {:?}", e)),
                            BinaryOp::Sub => self.builder.build_float_sub(l, r, "fsub")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float op: {:?}", e)),
                            BinaryOp::Mul => self.builder.build_float_mul(l, r, "fmul")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float op: {:?}", e)),
                            BinaryOp::Div => self.builder.build_float_div(l, r, "fdiv")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float op: {:?}", e)),
                            BinaryOp::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, l, r, "feq")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            BinaryOp::NotEqual => self.builder.build_float_compare(FloatPredicate::ONE, l, r, "fne")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            BinaryOp::Less => self.builder.build_float_compare(FloatPredicate::OLT, l, r, "flt")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            BinaryOp::Greater => self.builder.build_float_compare(FloatPredicate::OGT, l, r, "fgt")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            BinaryOp::LessEqual => self.builder.build_float_compare(FloatPredicate::OLE, l, r, "fle")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            BinaryOp::GreaterEqual => self.builder.build_float_compare(FloatPredicate::OGE, l, r, "fge")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build float compare: {:?}", e)),
                            _ => Err(format!("Unsupported float operator: {:?}", op)),
                        }
                    }
                    _ => Err("Type mismatch in binary operation".to_string()),
                }
            }
            HirExpr::Unary(op, expr) => {
                let val = self.codegen_expr(expr)?;
                
                match op {
                    UnaryOp::Neg => {
                        if let BasicValueEnum::IntValue(iv) = val {
                            self.builder.build_int_neg(iv, "neg")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build neg: {:?}", e))
                        } else {
                            Err("Negation only supported for integers".to_string())
                        }
                    }
                    UnaryOp::Not => {
                        if let BasicValueEnum::IntValue(iv) = val {
                            let zero = iv.get_type().const_zero();
                            self.builder.build_int_compare(IntPredicate::EQ, iv, zero, "not")
                                .map(|v| v.as_basic_value_enum())
                                .map_err(|e| format!("Failed to build not: {:?}", e))
                        } else {
                            Err("Not only supported for integers".to_string())
                        }
                    }
                    _ => Err(format!("Unsupported unary operator: {:?}", op)),
                }
            }
            HirExpr::Call(name, args) => {
                let func = self.module.get_function(name)
                    .ok_or_else(|| format!("Unknown function: {}", name))?;
                
                let arg_vals: Vec<BasicMetadataValueEnum> = args.iter()
                    .map(|a| self.codegen_expr(a).map(|v| v.into()))
                    .collect::<Result<Vec<_>, _>>()?;
                
                let call_site = self.builder.build_call(func, &arg_vals, "call")
                    .map_err(|e| format!("Failed to build call: {:?}", e))?;
                
                // If function returns void, return a dummy i32 value
                // This is a workaround for void function calls used in expression position
                if let Some(val) = call_site.try_as_basic_value().left() {
                    Ok(val)
                } else {
                    // Return a dummy value for void functions
                    Ok(self.context.i32_type().const_zero().as_basic_value_enum())
                }
            }
            HirExpr::Assign(lhs, rhs) => {
                let rhs_val = self.codegen_expr(rhs)?;
                
                if let HirExpr::Variable(name) = lhs.as_ref() {
                    let (ptr, _) = self.variables.get(name)
                        .ok_or_else(|| format!("Unknown variable: {}", name))?;
                    
                    self.builder.build_store(*ptr, rhs_val)
                        .map_err(|e| format!("Failed to store: {:?}", e))?;
                    
                    Ok(rhs_val)
                } else {
                    Err("Can only assign to variables".to_string())
                }
            }
            _ => Err("Expression codegen not fully implemented".to_string()),
        }
    }
    
    fn llvm_type(&self, ty: &Type) -> Result<Option<BasicTypeEnum<'ctx>>, String> {
        match ty {
            Type::Void => Ok(None),
            Type::Bool => Ok(Some(self.context.bool_type().as_basic_type_enum())),
            Type::Char | Type::I8 => Ok(Some(self.context.i8_type().as_basic_type_enum())),
            Type::I16 => Ok(Some(self.context.i16_type().as_basic_type_enum())),
            Type::I32 => Ok(Some(self.context.i32_type().as_basic_type_enum())),
            Type::I64 | Type::ISize => Ok(Some(self.context.i64_type().as_basic_type_enum())),
            Type::U8 => Ok(Some(self.context.i8_type().as_basic_type_enum())),
            Type::U16 => Ok(Some(self.context.i16_type().as_basic_type_enum())),
            Type::U32 => Ok(Some(self.context.i32_type().as_basic_type_enum())),
            Type::U64 | Type::USize => Ok(Some(self.context.i64_type().as_basic_type_enum())),
            Type::F32 => Ok(Some(self.context.f32_type().as_basic_type_enum())),
            Type::F64 => Ok(Some(self.context.f64_type().as_basic_type_enum())),
            Type::Pointer(inner, _) => {
                let inner_ty = if let Some(t) = self.llvm_type(inner)? {
                    t
                } else {
                    // Void pointer -> i8*
                    self.context.i8_type().as_basic_type_enum()
                };
                Ok(Some(inner_ty.ptr_type(AddressSpace::default()).as_basic_type_enum()))
            }
            Type::Array(elem_ty, size) => {
                if let Some(s) = size {
                    let elem = self.llvm_type(elem_ty)?
                        .ok_or("Cannot create array of void")?;
                    Ok(Some(elem.array_type(*s as u32).as_basic_type_enum()))
                } else {
                    Err("Unsized arrays not supported in codegen".to_string())
                }
            }
            _ => Err(format!("Type conversion not implemented: {:?}", ty)),
        }
    }
    
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
    
    pub fn print_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
    
    pub fn write_to_file(&self, path: &std::path::Path) -> Result<(), String> {
        self.module.print_to_file(path)
            .map_err(|e| format!("Failed to write module: {:?}", e))
    }
}

