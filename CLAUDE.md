# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Infra is a modern, Python-like programming language built in Rust. It features optional static typing, a clean syntax, and supports both direct AST interpretation and bytecode compilation for high performance execution.

## Essential Commands

### Building and Running
```bash
# Build the compiler (debug mode)
cargo build

# Build optimized release version
cargo build --release

# Run an Infra source file
cargo run -- hello.if

# Or after building:
./target/release/infra hello.if

# Start interactive REPL
cargo run -- --repl

# Quick syntax/type checking
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test module
cargo test backend::tests

# Run tests with output
cargo test -- --nocapture
```

### Development Tools
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security vulnerabilities
cargo audit

# Run specific test with output
cargo test test_name -- --nocapture

# Build and run with optimizations
cargo build --release && ./target/release/infra example.if
```

## Architecture Overview

The codebase follows a clean modular architecture:

### Core Module (`src/core/`)
- **ast.rs**: Abstract Syntax Tree definitions (expressions, statements, program structure)
- **value.rs**: Value type system supporting numbers, strings, arrays, objects, functions, and nil
- **error.rs**: Comprehensive error handling framework

### Frontend Module (`src/frontend/`)
- **lexer.rs**: Tokenizes source code into tokens
- **parser.rs**: Recursive descent parser that builds AST from tokens
- **token.rs**: Token type definitions and lexical structure

### Backend Module (`src/backend/`)
- **interpreter.rs**: Direct AST execution engine
- **bytecode.rs**: Bytecode compilation from AST
- **vm.rs**: Virtual machine for bytecode execution
- **environment.rs**: Runtime environment and scope management
- **evaluator.rs**: Expression evaluation system
- **module_system.rs**: Module loading and import functionality

### CLI Module (`src/cli/`)
- **runner.rs**: File execution and program evaluation
- **repl.rs**: Interactive read-eval-print loop

### Standard Library (`src/stdlib/`)
- **math.rs**: Mathematical functions and operations
- **string.rs**: String manipulation utilities
- **array.rs**: Array operations and methods
- **io.rs**: Input/output operations

### Utils Module (`src/utils/`)
- **error_reporter.rs**: User-friendly error reporting and formatting
- **version.rs**: Version information management

## Language Execution Pipeline

Infra supports dual execution modes:

1. **Direct Interpretation**: Source → Lexer → Parser → AST → Interpreter
2. **Bytecode Compilation**: Source → Lexer → Parser → AST → Bytecode Compiler → VM

Both paths support the same language features, allowing flexibility between development speed and runtime performance.

## File Extensions

- Use `.if` for Infra source files
- Example: `hello.if`, `program.if`

## Type System

Infra features an optional static typing system with:
- Type inference capabilities
- Explicit type annotations support  
- Comprehensive error reporting for type mismatches
- Type-related test infrastructure in the root directory

### Type Testing
- `test_type_system.if`: Core type system functionality
- `comprehensive_type_tests.if`: Advanced type scenarios
- `test_type_inference.if`: Type inference testing
- `type_error_tests.if`: Type error validation

## Development Workflow

1. **Making Changes**:
   - Start with `cargo check` to verify syntax
   - Use `cargo test` to ensure existing functionality works
   - Test with both interpreter and bytecode paths
   - Validate type system with `.if` test files

2. **Testing New Features**:
   - Add test cases to `src/backend/tests.rs`
   - Create example `.if` files for integration testing
   - Test with REPL for interactive validation
   - Add type system tests when applicable

3. **Common Development Tasks**:
   - **Adding new language features**: Update AST in `core/ast.rs`, implement parsing in `frontend/parser.rs`, and add execution logic in `backend/`
   - **Standard library functions**: Add to appropriate `stdlib/` module and register in the environment
   - **Error handling**: Define new error types in `core/error.rs` and implement user-friendly reporting in `utils/error_reporter.rs`
   - **Type system features**: Add to type definitions and update inference logic

## Tooling and Development Environment

### VS Code Extension (`vscode-extension/`)
- Syntax highlighting for `.if` files
- Code completion and IntelliSense
- Error diagnostics and formatting
- Custom snippets and language configuration

### Language Server (`infra-language-server/`)
- Provides language intelligence to various editors
- Supports real-time error checking
- Offers completion and hover information

### Tree-sitter Integration (`tree-sitter-infra/`)
- Advanced parsing infrastructure
- Enables better syntax analysis
- Supports language queries and transformations

## Development Status

### Active Development Areas
- **Bytecode Compiler**: Refactoring from `bytecode.rs` to `bytecode_new.rs`
- **Virtual Machine**: Improving VM implementation with `vm_new.rs`
- **Performance Optimization**: Moving AST interpretation to bytecode compilation
- **Type System**: Implementing optional static typing with inference

### Code Evolution
The codebase maintains parallel implementations during active development.
New versions are typically named with `_new` suffix until stable.

## Key Data Structures

### Value System
The `Value` enum in `src/core/value.rs` represents all runtime data types:
- Primitives: `Number`, `String`, `Boolean`, `Nil`
- Collections: `Array`, `Object` (key-value maps)
- Functions: Native functions and user-defined functions

### AST Structure
- **Statements**: Variable declarations, function definitions, control flow, expressions
- **Expressions**: Literals, variables, operations, function calls, conditional expressions

### Runtime Environment
Manages variable scoping, function definitions, and module imports through a nested environment system.

## Testing Strategy

### Unit Testing
- Located in `src/backend/tests.rs`
- Tests for backend functionality only
- Run with `cargo test`

### Integration Testing
- `.if` files in root directory serve as integration tests
- Test files prefixed with `test_` for organization
- Run with `cargo run -- test_file.if`

### Type System Testing
- Multiple specialized test files for type validation
- Tests both type inference and error reporting
- Critical for language reliability

Current testing focuses on core functionality:
- Arithmetic operations and expression evaluation
- Variable assignment and scope management
- Basic control flow structures
- Bytecode compilation and VM execution
- Type system validation and inference

## Configuration and Deployment

### Package Installation
- **Chocolatey**: Windows package manager support (`infra.rb`, `brew-infra.rb`)
- **Homebrew**: macOS package manager support
- **Shell scripts**: Direct installation for Linux/macOS (`install.sh`)
- **Docker**: Containerized deployment support (`docker-compose.yml`)

### Development Configuration
- VS Code workspace settings in `.vscode/`
- Rust configuration via `Cargo.toml`
- Package specifications for distribution (`infra.nuspec`)

## Module Dependencies

- **Core**: Used by all other modules (foundational data structures)
- **Frontend**: Independent of backend (clean separation)
- **Backend**: Depends on Core and Frontend
- **CLI**: Depends on all other modules
- **Stdlib**: Extends the backend with native functions