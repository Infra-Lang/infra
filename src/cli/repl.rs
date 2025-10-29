use crate::cli::Runner;

use std::io::{self, Write};

pub struct Repl {
    runner: Runner,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            runner: Runner::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Infra Programming Language v0.1.0");
        println!("Interactive REPL - Type 'exit', 'quit', or Ctrl+C to quit");
        println!("Type 'help' for commands or 'clear' to reset environment");
        println!();

        loop {
            print!("infra> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    if input.is_empty() {
                        continue;
                    }

                    match input {
                        "exit" | "quit" => {
                            println!("Goodbye!");
                            break;
                        }
                        "help" => {
                            self.show_help();
                            continue;
                        }
                        "clear" => {
                            self.runner.reset_interpreter();
                            println!("Environment cleared.");
                            continue;
                        }
                        "env" => {
                            self.show_environment();
                            continue;
                        }
                        _ => {
                            if let Err(err) = self.runner.execute_code(input) {
                                eprintln!("{}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
        }
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  help    - Show this help message");
        println!("  clear   - Reset the environment (clear all variables)");
        println!("  env     - Show current environment variables");
        println!("  exit    - Exit the REPL");
        println!("  quit    - Exit the REPL");
        println!();
        println!("Language syntax examples:");
        println!("  let x = 42");
        println!("  let name = \"Hello, World!\"");
        println!("  print(x + 10)");
        println!("  if x > 30: print(\"Large number\")");
        println!("  for i in range(0, 5): print(i)");
        println!();
    }

    fn show_environment(&self) {
        let env = self.runner.get_interpreter().get_environment();
        let size = env.size();

        if size == 0 {
            println!("Environment is empty (no variables defined)");
        } else {
            println!("Environment contains {} variable(s)", size);
            // Note: We'd need to expose the variables HashMap to show them
            // This is a design decision - do we want to expose internal state?
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
