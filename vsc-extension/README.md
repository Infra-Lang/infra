# Infra Language Support

A comprehensive VS Code extension providing language support for the Infra programming language.

## Features

### 🎨 **Syntax Highlighting**
- Beautiful syntax highlighting for `.if` and `.infra` files
- Support for all Infra language constructs:
  - Keywords: `let`, `function`, `if`, `else`, `for`, `while`, `try`, `catch`
  - Types: `number`, `string`, `boolean`, `array`, `object`
  - Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `&&`, `||`, `->`
  - Standard library functions: `math.*`, `string.*`, `array.*`, `io.*`

### ⚡ **Quick Actions**
- **Run File**: Execute Infra files directly from VS Code
- **Check Syntax**: Validate syntax without execution
- Context menu integration for `.if` and `.infra` files

### 🐛 **Error Detection**
- Real-time syntax error checking
- Integration with Infra interpreter for accurate diagnostics
- Errors appear in the Problems panel with line/column information

### 🛠️ **Language Features**
- Bracket matching and auto-closing
- Smart indentation
- Comment toggling with `#`
- Code folding support

### 🎨 **Custom File Icons**
- Beautiful custom icons for `.if` and `.infra` files automatically enabled
- Dark and light theme variants
- Distinctive file recognition in Explorer  
- Professional document-style icons with Infra branding
- **No configuration required** - icons appear immediately after installation

## Usage

### Running Infra Files
1. Open any `.if` or `.infra` file
2. Click the play button in the editor title bar, or
3. Right-click and select "Run Infra File"

### Syntax Checking
1. Open any `.if` or `.infra` file
2. Right-click and select "Check Syntax"
3. View results in the integrated terminal

### Error Detection
- Errors appear automatically in the Problems panel
- Save the file to trigger error checking
- Hover over error markers for details

## Infra Language Overview

Infra is a modern programming language with Python-like syntax and static typing support:

```infra
# Variables with type annotations
let score: number = 95
let message: string = "Hello, Infra!"
let active: boolean = true

# Functions with type checking
function calculateArea(length: number, width: number) -> number:
    return length * width

# Data structures
let numbers: [number] = [1, 2, 3, 4, 5]
let person: {name: string, age: number} = {
    name: "Alice",
    age: 30
}

# Control flow
if score >= 90:
    print("Excellent work!")
else:
    print("Keep trying!")

# Standard library
let result = math.sqrt(16)
let length = string.length("Hello")
array.push(numbers, 6)
```

## Requirements

- **Infra Interpreter**: The extension requires the Infra language interpreter to be available in the parent directory (`../`)
- **Cargo**: Used to run the Rust-based Infra interpreter

## Extension Settings

This extension contributes the following settings:

* Currently no configurable settings (coming in future versions)

## Known Issues

- Error detection requires saving the file to trigger
- Performance may be slower for very large files
- Requires Infra interpreter in specific directory structure

## Release Notes

### 1.0.0

Initial release of Infra Language Support:
- Complete syntax highlighting for `.if` and `.infra` files
- Integration with Infra interpreter for running files
- Real-time error detection and reporting
- Language configuration for smart editing
- **Automatic custom file icons** (no setup required)
- Code snippets for common Infra patterns

## Development

### Project Structure
```
├── syntaxes/
│   └── infra.tmLanguage.json     # TextMate grammar
├── language-configuration.json   # Language configuration
├── src/
│   └── extension.ts              # Main extension logic
└── package.json                  # Extension manifest
```

### Building from Source
```bash
npm install
npm run compile
```

### Testing
```bash
npm test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This extension is licensed under the MIT License.

## Following extension guidelines

Ensure that you've read through the extensions guidelines and follow the best practices for creating your extension.

* [Extension Guidelines](https://code.visualstudio.com/api/references/extension-guidelines)

## Working with Markdown

You can author your README using Visual Studio Code. Here are some useful editor keyboard shortcuts:

* Split the editor (`Cmd+\` on macOS or `Ctrl+\` on Windows and Linux).
* Toggle preview (`Shift+Cmd+V` on macOS or `Shift+Ctrl+V` on Windows and Linux).
* Press `Ctrl+Space` (Windows, Linux, macOS) to see a list of Markdown snippets.

## For more information

* [Visual Studio Code's Markdown Support](http://code.visualstudio.com/docs/languages/markdown)
* [Markdown Syntax Reference](https://help.github.com/articles/markdown-basics/)

**Enjoy!**
