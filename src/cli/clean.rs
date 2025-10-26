use anyhow::Result;
use std::path::PathBuf;
use std::fs;

pub fn execute(path: PathBuf) -> Result<()> {
    println!("🧹 Cleaning build artifacts...");
    
    let build_dir = path.join("build");
    
    if build_dir.exists() {
        fs::remove_dir_all(&build_dir)?;
        println!("  Removed: {}", build_dir.display());
    }
    
    println!("✅ Clean complete");
    
    Ok(())
}

