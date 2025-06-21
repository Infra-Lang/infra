use std::env;
use std::process;

mod core;
mod frontend;
mod backend;
mod cli;
mod utils;
mod stdlib;

use cli::{Runner, Repl};
use utils::{version_info, ErrorReporter};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_usage(&args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--repl" | "-r" => {
            run_repl();
        }
        "--version" | "-v" => {
            println!("{}", version_info());
        }
        "--help" | "-h" => {
            show_help(&args[0]);
        }
        filename => {
            run_file(filename);
        }
    }
}

fn run_file(filename: &str) {
    let mut runner = Runner::new();
    let mut error_reporter = ErrorReporter::new();
    
    if let Err(err) = runner.run_file(filename) {
        error_reporter.report_error(&err);
        process::exit(1);
    }
}

fn run_repl() {
    let mut repl = Repl::new();
    repl.run();
}

fn show_usage(program_name: &str) {
    println!("{}", version_info());
    println!("Usage: {} [OPTIONS] <file.infra>", program_name);
    println!("   or: {} --repl", program_name);
    println!();
    println!("Options:");
    println!("  -r, --repl      Start interactive REPL");
    println!("  -v, --version   Show version information");
    println!("  -h, --help      Show this help message");
}

fn show_help(program_name: &str) {
    show_usage(program_name);
    println!();
    println!("Examples:");
    println!("  {} program.infra     # Run a file", program_name);
    println!("  {} --repl            # Start interactive mode", program_name);
    println!();
    println!("For more information, visit: https://github.com/infra-lang/infra");
}
