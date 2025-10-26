use anyhow::Result;
use std::path::PathBuf;
use std::fs;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::types::TypeChecker;

pub fn execute(input: PathBuf) -> Result<()> {
    println!("🔍 Checking {}...", input.display());
    
    // Read source file
    let source = fs::read_to_string(&input)?;
    
    // Lex
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program()
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
    
    // Type check
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&ast)
        .map_err(|e| anyhow::anyhow!("Type error: {}", e))?;
    
    println!("✅ No errors found");
    
    Ok(())
}

