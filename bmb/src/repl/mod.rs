//! REPL (Read-Eval-Print Loop) for BMB

use crate::interp::Interpreter;
use crate::lexer::tokenize;
use crate::parser::parse;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use std::path::PathBuf;

const PROMPT: &str = "> ";
const HISTORY_FILE: &str = ".bmb_history";

/// REPL state
pub struct Repl {
    editor: DefaultEditor,
    interpreter: Interpreter,
    history_path: Option<PathBuf>,
}

impl Repl {
    /// Create a new REPL
    pub fn new() -> RlResult<Self> {
        let editor = DefaultEditor::new()?;
        let interpreter = Interpreter::new();

        // Try to find history file in home directory
        let history_path = dirs_home().map(|h| h.join(HISTORY_FILE));

        let mut repl = Repl {
            editor,
            interpreter,
            history_path,
        };

        // Load history if available
        if let Some(ref path) = repl.history_path {
            let _ = repl.editor.load_history(path);
        }

        Ok(repl)
    }

    /// Run the REPL
    pub fn run(&mut self) -> RlResult<()> {
        println!("BMB REPL v0.3");
        println!("Type :help for help, :quit to exit.\n");

        loop {
            match self.editor.readline(PROMPT) {
                Ok(line) => {
                    let line = line.trim();

                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    let _ = self.editor.add_history_entry(line);

                    // Handle commands
                    if line.starts_with(':') {
                        if self.handle_command(line) {
                            break;
                        }
                        continue;
                    }

                    // Try to parse and evaluate
                    self.eval_input(line);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye!");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                    break;
                }
            }
        }

        // Save history
        if let Some(ref path) = self.history_path {
            let _ = self.editor.save_history(path);
        }

        Ok(())
    }

    /// Handle REPL commands (starting with :)
    fn handle_command(&mut self, cmd: &str) -> bool {
        match cmd {
            ":quit" | ":q" | ":exit" => {
                println!("Goodbye!");
                true
            }
            ":help" | ":h" | ":?" => {
                self.print_help();
                false
            }
            ":clear" => {
                print!("\x1B[2J\x1B[1;1H");
                false
            }
            _ => {
                println!("Unknown command: {cmd}");
                println!("Type :help for help.");
                false
            }
        }
    }

    /// Print help message
    fn print_help(&self) {
        println!("BMB REPL Commands:");
        println!("  :help, :h, :?   Show this help");
        println!("  :quit, :q       Exit the REPL");
        println!("  :clear          Clear the screen");
        println!();
        println!("You can enter:");
        println!("  - Expressions: 1 + 2, if true then 1 else 2");
        println!("  - Function definitions: fn add(a: i32, b: i32) -> i32 = a + b;");
        println!("  - Function calls: add(1, 2)");
        println!();
        println!("Built-in functions:");
        println!("  println(x)      Print value with newline");
        println!("  print(x)        Print value without newline");
        println!("  assert(cond)    Assert condition is true");
        println!("  abs(n)          Absolute value");
        println!("  min(a, b)       Minimum of two values");
        println!("  max(a, b)       Maximum of two values");
    }

    /// Evaluate user input
    fn eval_input(&mut self, input: &str) {
        // Wrap expression in a dummy function if it's not a function definition
        let source = if input.starts_with("fn ") {
            input.to_string()
        } else {
            // Wrap expression as a function body for evaluation
            format!("fn __repl__() -> i64 = {input};")
        };

        // Tokenize
        let tokens = match tokenize(&source) {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("Lexer error: {}", err.message());
                return;
            }
        };

        // Parse
        match parse("<repl>", &source, tokens) {
            Ok(program) => {
                // Load any function definitions
                self.interpreter.load(&program);

                // Run the program (which will call __repl__ or main)
                match self.interpreter.run(&program) {
                    Ok(value) => {
                        // Don't print Unit values (like from println)
                        if !matches!(value, crate::interp::Value::Unit) {
                            println!("{value}");
                        }
                    }
                    Err(err) => {
                        eprintln!("Runtime error: {}", err.message);
                    }
                }
            }
            Err(err) => {
                eprintln!("Parse error: {}", err.message());
            }
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new().expect("Failed to create REPL")
    }
}

/// Get home directory
fn dirs_home() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
