# VS Code Setup for Infra Language

## Prerequisites

- Visual Studio Code 1.85.0 or later
- Git

## Installation Methods

### Method 1: Install from Marketplace (Recommended)

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Infra Language"
4. Click Install

### Method 2: Install from VSIX

1. Download the latest `.vsix` file from [GitHub Releases](https://github.com/Infra-Lang/infra/releases)
2. Open VS Code
3. Go to Extensions (Ctrl+Shift+X)
4. Click the "..." menu and select "Install from VSIX..."
5. Select the downloaded `.vsix` file

### Method 3: Manual Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Infra-Lang/infra.git
   cd infra/editors/vscode-extension
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Compile TypeScript:
   ```bash
   npm run compile
   ```

4. Install extension:
   ```bash
   code --install-extension .
   ```

## Configuration

### Language Server Settings

Open VS Code settings (File > Preferences > Settings) and add:

```json
{
  "infra.lsp.enabled": true,
  "infra.lsp.path": "",
  "infra.formatting.enabled": true
}
```

### Workspace Settings

Create `.vscode/settings.json` in your workspace:

```json
{
  "files.associations": {
    "*.if": "infra"
  },
  "[infra]": {
    "editor.defaultFormatter": "infra-vscode.infra-format",
    "editor.formatOnSave": true,
    "editor.tabSize": 2,
    "editor.insertSpaces": true
  },
  "infra.lsp.enabled": true,
  "infra.lsp.path": "${workspaceFolder}/editors/lsp-server/target/debug/infra-lsp.exe"
}
```

## Features

### Syntax Highlighting
- Full syntax highlighting for Infra code
- Bracket matching and auto-close
- Comment toggling (`Ctrl+/`)
- Color-coded keywords, strings, comments, etc.

### Language Server Support
- **Code Completion**: Intelligent code suggestions
- **Hover Information**: Documentation on hover
- **Go to Definition**: Jump to function/class definitions
- **Find References**: Find all usages of symbols
- **Document Symbols**: Navigate to functions and classes
- **Workspace Symbols**: Search across all files
- **Code Formatting**: Auto-format code on save
- **Error Checking**: Real-time syntax and type errors
- **Signature Help**: Function parameter hints

### Snippets
Type these shortcuts to generate code:

| Trigger | Description |
|---------|-------------|
| `func` | Function definition |
| `class` | Class definition |
| `let` | Variable declaration |
| `if` | If-else statement |
| `for` | For loop |
| `while` | While loop |
| `async` | Async function |
| `try` | Try-catch block |
| `import` | Import statement |
| `export` | Export statement |

### File Associations
- `.if` files are automatically recognized as Infra files
- Files named with `.infra` extension are also recognized

## Settings Reference

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `infra.lsp.enabled` | boolean | `true` | Enable/disable language server |
| `infra.lsp.path` | string | `""` | Path to LSP executable (auto-detected if empty) |
| `infra.formatting.enabled` | boolean | `true` | Enable/disable code formatting |

## Key Bindings

| Shortcut | Description |
|---------|-------------|
| `Ctrl+/` | Toggle line comment |
| `Ctrl+]` | Go to definition |
| `F12` | Go to definition |
| `Shift+F12` | Find references |
| `Ctrl+Shift+F12` | Find all references |
| `F2` | Rename symbol |
| `Ctrl+Shift+I` | Peek definition |
| `Ctrl+.` | Quick fix |
| `Ctrl+Shift+.` | Show problems |
| `Ctrl+,` | Open settings |
| `Ctrl+K Ctrl+S` | Save all |

## Troubleshooting

### Language Server Not Starting

1. Check VS Code output panel (View > Output)
2. Look for "Infra Language Server" channel
3. Ensure `infra-lsp` is in your PATH or specify `infra.lsp.path`

### Syntax Highlighting Not Working

1. Reload VS Code window (Ctrl+R)
2. Check file association settings
3. Verify `.if` files are recognized as Infra

### Formatting Issues

1. Check that `infra.formatting.enabled` is `true`
2. Try manual formatting: `Ctrl+Shift+I`
3. Check for syntax errors preventing formatting

### Extension Not Loading

1. Check that VS Code version is >= 1.85.0
2. Disable other extensions that might conflict
3. Try running VS Code with extensions disabled: `code --disable-extensions`

## Development Mode

If you want to work on the extension itself:

```bash
cd infra/editors/vscode-extension
npm install
npm run compile
npm run watch
```

Then run VS Code with the extension loaded:
```bash
code --extensionDevelopmentPath=.
```