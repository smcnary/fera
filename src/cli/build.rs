use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use inkwell::context::Context;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::types::TypeChecker;
use crate::hir::HirBuilder;
use crate::codegen::CodeGen;

pub fn execute(
    input: PathBuf,
    release: bool,
    output: Option<PathBuf>,
    target: Option<String>,
    link: Vec<String>,
    opt_level: u8,
) -> Result<()> {
    println!("🔨 Building {}...", input.display());
    
    // Read source file
    let source = fs::read_to_string(&input)?;
    
    // Lex
    println!("  Lexing...");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parse
    println!("  Parsing...");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program()
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    // Type check
    println!("  Type checking...");
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&ast)
        .map_err(|e| anyhow::anyhow!("Type error: {}", e))?;
    
    // Lower to HIR
    println!("  Lowering to HIR...");
    let mut hir_builder = HirBuilder::new();
    let hir = hir_builder.lower_program(&ast)
        .map_err(|e| anyhow::anyhow!("HIR error: {}", e))?;
    
    // Generate LLVM IR
    println!("  Generating LLVM IR...");
    let context = Context::create();
    let module_name = input.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");
    let mut codegen = CodeGen::new(&context, module_name);
    codegen.codegen_program(&hir)
        .map_err(|e| anyhow::anyhow!("Codegen error: {}", e))?;
    
    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension("");
        path
    });
    
    // Write LLVM IR for debugging
    if !release {
        let mut ir_path = output_path.clone();
        ir_path.set_extension("ll");
        codegen.write_to_file(&ir_path)
            .map_err(|e| anyhow::anyhow!("Failed to write IR: {}", e))?;
        println!("  LLVM IR written to: {}", ir_path.display());
    }
    
    // Initialize LLVM target
    Target::initialize_native(&InitializationConfig::default())
        .map_err(|e| anyhow::anyhow!("Failed to initialize target: {}", e))?;
    
    let target_triple = if let Some(t) = target {
        inkwell::targets::TargetTriple::create(&t)
    } else {
        TargetMachine::get_default_triple()
    };
    
    let target = Target::from_triple(&target_triple)
        .map_err(|e| anyhow::anyhow!("Failed to create target: {}", e))?;
    
    let opt = if release || opt_level >= 2 {
        OptimizationLevel::Aggressive
    } else if opt_level == 1 {
        OptimizationLevel::Less
    } else {
        OptimizationLevel::None
    };
    
    let target_machine = target.create_target_machine(
        &target_triple,
        "generic",
        "",
        opt,
        RelocMode::Default,
        CodeModel::Default,
    ).ok_or_else(|| anyhow::anyhow!("Failed to create target machine"))?;
    
    // Generate object file
    println!("  Generating object file...");
    let mut obj_path = output_path.clone();
    obj_path.set_extension("o");
    
    target_machine.write_to_file(codegen.get_module(), FileType::Object, &obj_path)
        .map_err(|e| anyhow::anyhow!("Failed to write object file: {}", e))?;
    
    // Link
    println!("  Linking...");
    let mut link_cmd = std::process::Command::new("cc");
    link_cmd.arg(&obj_path);
    link_cmd.arg("-o");
    link_cmd.arg(&output_path);
    
    for lib in &link {
        link_cmd.arg(format!("-l{}", lib));
    }
    
    let status = link_cmd.status()?;
    
    if !status.success() {
        anyhow::bail!("Linking failed");
    }
    
    // Clean up object file
    fs::remove_file(&obj_path).ok();
    
    println!("✅ Build complete: {}", output_path.display());
    
    Ok(())
}

