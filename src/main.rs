mod lexer;
mod parser;
mod ast;
mod types;
mod hir;
mod codegen;
mod cli;
mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fera")]
#[command(about = "Fera programming language compiler and toolchain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a Fera project or file
    Build {
        /// Input file or project directory
        input: PathBuf,
        
        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Target triple (e.g., x86_64-linux-gnu)
        #[arg(short, long)]
        target: Option<String>,
        
        /// Libraries to link against
        #[arg(short, long)]
        link: Vec<String>,
        
        /// Optimization level (0-3)
        #[arg(short = 'O', default_value = "0")]
        opt_level: u8,
    },
    
    /// Build and run a Fera project or file
    Run {
        /// Input file or project directory
        input: PathBuf,
        
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        
        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Run tests
    Test {
        /// Test filter pattern
        filter: Option<String>,
    },
    
    /// Format Fera source files
    Fmt {
        /// Files to format
        files: Vec<PathBuf>,
        
        /// Check formatting without modifying files
        #[arg(short, long)]
        check: bool,
    },
    
    /// Type-check and lint without building
    Check {
        /// Input file or project directory
        input: PathBuf,
    },
    
    /// Remove build artifacts
    Clean {
        /// Project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Generate documentation
    Doc {
        /// Open documentation in browser after generation
        #[arg(long)]
        open: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { input, release, output, target, link, opt_level } => {
            cli::build::execute(input, release, output, target, link, opt_level)
        }
        Commands::Run { input, release, args } => {
            cli::run::execute(input, release, args)
        }
        Commands::Test { filter } => {
            cli::test::execute(filter)
        }
        Commands::Fmt { files, check } => {
            cli::fmt::execute(files, check)
        }
        Commands::Check { input } => {
            cli::check::execute(input)
        }
        Commands::Clean { path } => {
            cli::clean::execute(path)
        }
        Commands::Doc { open } => {
            cli::doc::execute(open)
        }
    }
}

