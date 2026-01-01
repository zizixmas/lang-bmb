//! BMB Compiler CLI

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bmb", version, about = "BMB Compiler - AI-Native Language")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a BMB program
    Run {
        /// Source file to run
        file: PathBuf,
    },
    /// Start interactive REPL
    Repl,
    /// Type check a BMB source file
    Check {
        /// Source file to check
        file: PathBuf,
    },
    /// Verify contracts (pre/post conditions) using SMT solver
    Verify {
        /// Source file to verify
        file: PathBuf,
        /// Path to Z3 executable (default: z3)
        #[arg(long, default_value = "z3")]
        z3_path: String,
        /// Timeout in seconds
        #[arg(long, short = 't', default_value = "10")]
        timeout: u32,
    },
    /// Parse and dump AST (debug)
    Parse {
        /// Source file to parse
        file: PathBuf,
    },
    /// Tokenize and dump tokens (debug)
    Tokens {
        /// Source file to tokenize
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Run { file } => run_file(&file),
        Command::Repl => start_repl(),
        Command::Check { file } => check_file(&file),
        Command::Verify { file, z3_path, timeout } => verify_file(&file, &z3_path, timeout),
        Command::Parse { file } => parse_file(&file),
        Command::Tokens { file } => tokenize_file(&file),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let filename = path.display().to_string();

    // Tokenize
    let tokens = bmb::lexer::tokenize(&source)?;

    // Parse
    let ast = bmb::parser::parse(&filename, &source, tokens)?;

    // Type check first
    let mut checker = bmb::types::TypeChecker::new();
    checker.check_program(&ast)?;

    // Run with interpreter
    let mut interpreter = bmb::interp::Interpreter::new();
    interpreter.load(&ast);

    match interpreter.run(&ast) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Runtime error: {}", e.message);
            std::process::exit(1);
        }
    }
}

fn start_repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = bmb::repl::Repl::new()?;
    repl.run()?;
    Ok(())
}

fn check_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let filename = path.display().to_string();

    // Tokenize
    let tokens = bmb::lexer::tokenize(&source)?;

    // Parse
    let ast = bmb::parser::parse(&filename, &source, tokens)?;

    // Type check
    let mut checker = bmb::types::TypeChecker::new();
    checker.check_program(&ast)?;

    println!("✓ {} type checks successfully", filename);
    Ok(())
}

fn verify_file(path: &PathBuf, z3_path: &str, timeout: u32) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let filename = path.display().to_string();

    // Tokenize
    let tokens = bmb::lexer::tokenize(&source)?;

    // Parse
    let ast = bmb::parser::parse(&filename, &source, tokens)?;

    // Type check first
    let mut checker = bmb::types::TypeChecker::new();
    checker.check_program(&ast)?;

    // Set up verifier
    let verifier = bmb::verify::ContractVerifier::new()
        .with_z3_path(z3_path)
        .with_timeout(timeout);

    // Check if solver is available
    if !verifier.is_solver_available() {
        eprintln!("Warning: Z3 solver not found at '{}'. Install Z3 or specify --z3-path.", z3_path);
        eprintln!("Skipping contract verification.");
        return Ok(());
    }

    // Verify contracts
    let report = verifier.verify_program(&ast);

    // Print report
    print!("{}", report);

    // Exit with error if any verification failed
    if !report.all_verified() {
        std::process::exit(1);
    }

    Ok(())
}

fn parse_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;
    let filename = path.display().to_string();

    let tokens = bmb::lexer::tokenize(&source)?;
    let ast = bmb::parser::parse(&filename, &source, tokens)?;

    println!("{}", serde_json::to_string_pretty(&ast)?);
    Ok(())
}

fn tokenize_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path)?;

    let tokens = bmb::lexer::tokenize(&source)?;
    for (tok, span) in &tokens {
        println!("{:?} @ {}..{}", tok, span.start, span.end);
    }

    Ok(())
}
