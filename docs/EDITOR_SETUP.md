# Editor Setup and Tooling for Infra

This guide covers setting up modern editor support for the Infra programming language, including syntax highlighting, language server protocol (LSP), and advanced features.

## 🚀 Quick Start

### VS Code (Recommended)
1. Install from Marketplace: Search "Infra Language"
2. Auto-enabled features: syntax highlighting, LSP, formatting
3. Ready to use immediately!

### Other Editors
See setup guides below for specific editors.

## 🛠️ Supported Editors

### Visual Studio Code
- **Status**: ✅ Fully Supported
- **Features**: Syntax highlighting, LSP, formatting, snippets
- **Installation**: Marketplace or manual installation
- **Documentation**: [VS Code Setup Guide](../editors/setup-vscode.md)

### Neovim
- **Status**: ✅ Supported with Tree-sitter + LSP
- **Features**: Syntax highlighting, LSP, completion
- **Installation**: Requires Tree-sitter parser build
- **Documentation**: [Neovim Setup Guide](../editors/setup-neovim.md)

### Emacs
- **Status**: ✅ Supported with Tree-sitter + LSP
- **Features**: Syntax highlighting, LSP, formatting
- **Installation**: Requires Tree-sitter parser build
- **Documentation**: [Emacs Setup Guide](../editors/setup-emacs.md)

### Sublime Text
- **Status**: ⚠️ Partial Support (syntax highlighting only)
- **Features**: Syntax highlighting (if available)
- **Installation**: Manual grammar file installation
- **Limitation**: No LSP support

### Vim/Neovim (Vimscript)
- **Status**: ⚠️ Basic Support
- **Features**: Basic syntax highlighting (if available)
- **Installation**: Manual plugin installation
- **Limitation**: No LSP support

## 📋 Features by Editor

| Feature | VS Code | Neovim | Emacs | Others |
|--------|---------|---------|-------|--------|
| Syntax Highlighting | ✅ | ✅ | ✅ | ⚠️ |
| Code Completion | ✅ | ✅ | ✅ | ⚠️ |
| LSP Support | ✅ | ✅ | ✅ | ❌ |
| Go to Definition | ✅ | ✅ | ✅ | ❌ |
| Find References | ✅ | ✅ | ✅ | ❌ |
| Hover Documentation | ✅ | ✅ | ✅ | ❌ |
| Code Formatting | ✅ | ✅ | ✅ | ❌ |
| Error Checking | ✅ | ✅ | ✅ | ❌ |
| Snippets | ✅ | ✅ | ✅ | ❌ |
| Auto-indentation | ✅ | ✅ | ✅ | ⚠️ |

## 🏗️ Language Server Protocol (LSP)

The Infra Language Server provides:

### Core Features
- **Code Completion**: Intelligent suggestions based on context
- **Hover Information**: Documentation for symbols
- **Go to Definition**: Navigate to function/class definitions
- **Find References**: Locate all usages of symbols
- **Document Symbols**: Navigate within files
- **Workspace Symbols**: Search across all files
- **Code Formatting**: Automatic code formatting
- **Error Checking**: Real-time syntax and type errors

### Language Features
- **Keyword Completion**: All Infra keywords
- **Function Completions**: Built-in and user-defined functions
- **Type Annotations**: Support for type hints
- **Import Resolution**: Module and import handling
- **Class Members**: Class method and property completion

### Installation

#### From Source
```bash
cd editors/lsp-server
cargo build --release
cp target/release/infra-lsp ~/.local/bin/
```

#### Package Installation (Future)
```bash
# Will be available when published to package managers
npm install -g infra-lsp
```

## 🎨 Syntax Highlighting

Based on Tree-sitter grammar for accurate parsing and highlighting.

### Supported Syntax Elements
- **Keywords**: All Infra keywords
- **Types**: Built-in types and user-defined types
- **Strings**: Single, double, raw, and f-strings
- **Comments**: Single-line and block comments
- **Numbers**: Integer, float, hex, binary, octal
- **Operators**: All arithmetic, comparison, logical operators
- **Punctuation**: Brackets, separators, terminators

### Color Scheme
- **Keywords**: Language keywords (blue)
- **Types**: Type annotations (teal)
- **Functions**: Built-in functions (yellow)
- **Strings**: String literals (green)
- **Comments**: Comments (gray)
- **Numbers**: Numeric literals (purple)
- **Variables**: Variables and identifiers (default)
- **Constants**: Constants in ALL_CAPS (orange)

## 🔧 Configuration

### VS Code Settings
```json
{
  "infra.lsp.enabled": true,
  "infra.lsp.path": "",
  "infra.formatting.enabled": true
}
```

### Neovim Configuration
```lua
lspconfig.infra_lsp.setup({
  capabilities = capabilities,
  cmd = { 'infra-lsp' },
  filetypes = { 'infra' },
  settings = {
    infra = {
      lsp = { enabled = true },
      formatting = { enabled = true }
    }
  }
})
```

### Emacs Configuration
```elisp
(lsp-register-client
 (make-lsp-client :new-connection
  (lsp-stdio-connection 'infra-lsp))
 :major-modes '(infra-mode)
 :server-id 'infra-lsp))
```

## 🚀 Advanced Features

### Code Snippets

Common Infra patterns:

#### Function Definition
```infra
function ${1:name}(${2:parameters}): ${3:return_type} {
    ${4:// body}
}
```

#### Class Definition
```infra
class ${1:name}${2::ParentClass} {
    function init(${3:parameters}) {
        ${4:// initialization}
    }
}
```

#### Variable Declaration
```infra
let ${1:name}: ${2:type} = ${3:value}
```

#### Conditional Statement
```infra
if ${1:condition} {
    ${2:// consequence}
} else {
    ${3:// alternative}
}
```

#### Loop
```infra
for ${1:item} in ${2:iterable} {
    ${3:// loop body}
}
```

### File Templates

#### Basic Program
```infra
// Infra programming language
// Author: ${1:author}
// Description: ${2:description}

function main() {
    ${3:// main code}
}

main()
```

#### Class Definition
```infra
class ${1:ClassName} {
    function init(${2:parameters}) {
        ${3:// constructor}
    }
    
    function ${4:method}(${5:parameters}): ${6:return_type} {
        ${7:// method implementation}
    }
}
```

## 🐛 Build Tools

### Tree-sitter Parser
```bash
cd editors/tree-sitter-infra
npm install
npm run build
```

### LSP Server
```bash
cd editors/lsp-server
cargo build --release
```

### VS Code Extension
```bash
cd editors/vscode-extension
npm install
npm run compile
npm run package
```

## 🔍 Troubleshooting

### General Issues

#### LSP Not Starting
1. Check that `infra-lsp` is in your PATH
2. Verify the executable has execute permissions
3. Check editor logs for error messages

#### Syntax Highlighting Not Working
1. Reload editor window
2. Check file association settings
3. Verify Tree-sitter parser is installed correctly

#### Performance Issues
1. Disable unused features in settings
2. Limit workspace size for large projects
3. Check system resources

### Editor-Specific Issues

#### VS Code
- **Extension Not Loading**: Check VS Code version >= 1.85.0
- **LSP Errors**: Check Output panel for Infra Language Server
- **Formatting Issues**: Ensure `infra.formatting.enabled` is true

#### Neovim
- **Tree-sitter Issues**: Run `:TSInstallSync infra`
- **LSP Issues**: Check `:LspInfo` for server status
- **Completion Issues**: Verify nvim-cmp configuration

#### Emacs
- **Tree-sitter Issues**: Rebuild parser with `tree-sitter-reload-grammar`
- **LSP Issues**: Check `lsp-ui-doc-show` for server status
- **Indentation Issues**: Verify `tree-sitter-indent-mode` is enabled

## 🤝 Contributing

### Adding New Features
1. Fork the repository
2. Implement feature in appropriate editor
3. Add tests if applicable
4. Update documentation
5. Submit pull request

### Testing Changes
- Test changes across multiple editors
- Ensure backward compatibility
- Update relevant documentation
- Verify LSP functionality

## 📚 Additional Resources

### Official Documentation
- [Infra Language Guide](LANGUAGE_GUIDE.md)
- [Standard Library Reference](STANDARD_LIBRARY.md)
- [Installation Guide](INSTALLATION.md)

### Editor Documentation
- [VS Code Setup](editors/setup-vscode.md)
- [Neovim Setup](editors/setup-neovim.md)
- [Emacs Setup](editors/setup-emacs.md)

### External Resources
- [Tree-sitter Documentation](https://tree-sitter.github.io/)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
- [VS Code Extension API](https://code.visualstudio.com/api)

## 📄 File Extensions

| Extension | Description |
|-----------|-------------|
| `.if` | Primary Infra source file |
| `.infra` | Alternative Infra source file |
| `.tree-sitter-infra.c` | Generated Tree-sitter parser |

## 🔖 Development Status

- **Syntax Highlighting**: ✅ Stable
- **LSP Server**: ✅ Beta (Core features complete)
- **VS Code Extension**: ✅ Beta
- **Neovim Support**: ✅ Beta
- **Emacs Support**: ✅ Beta
- **Tree-sitter Parser**: ✅ Stable
- **Code Formatting**: ✅ Basic implementation
- **Error Checking**: ✅ Basic implementation

## 🎯 Roadmap

### Short Term (Next 3 months)
- Enhanced error checking and diagnostics
- Improved code formatting rules
- More sophisticated code snippets
- Better integration with build tools
- Performance optimizations

### Long Term (Next 6 months)
- Debugging support
- Refactoring tools
- Unit test integration
- Workspace symbols improvements
- Multi-file refactoring

---

**Need Help?**  
- [GitHub Issues](https://github.com/Infra-Lang/infra/issues) - Report bugs
- [GitHub Discussions](https://github.com/Infra-Lang/infra/discussions) - General discussion
- [Documentation](https://github.com/Infra-Lang/infra/docs) - Read the docs

**Happy coding with Infra! 🚀**