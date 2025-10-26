use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

use super::build;

pub fn execute(input: PathBuf, release: bool, args: Vec<String>) -> Result<()> {
    // Build first
    let mut output = input.clone();
    output.set_extension("");
    
    build::execute(input, release, Some(output.clone()), None, vec![], 0)?;
    
    // Run the binary
    println!("\n🚀 Running {}...\n", output.display());
    
    let status = Command::new(&output)
        .args(&args)
        .status()?;
    
    if !status.success() {
        if let Some(code) = status.code() {
            println!("\nProcess exited with code: {}", code);
        } else {
            println!("\nProcess terminated by signal");
        }
    }
    
    Ok(())
}

