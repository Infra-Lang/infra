# Neovim Setup for Infra Language

## Prerequisites

- Neovim 0.7.0 or later
- Git
- Rust toolchain (for building Tree-sitter)

## Installation

### 1. Install Tree-sitter Parser

```bash
# Clone the infra repository
git clone https://github.com/Infra-Lang/infra.git
cd infra/editors/tree-sitter-infra

# Build and install the parser
npm install
npm run build
nvim --head -c "TSInstallSync infra"
```

### 2. Neovim Configuration

Add this to your `init.lua`:

```lua
-- Tree-sitter configuration
vim.filetype.add({
  extension = {
    if = 'infra',
  },
  pattern = {
    ['^.*%.infra$'] = 'infra',
  },
})

-- Tree-sitter highlights
require('nvim-treesitter.configs').setup {
  highlight = {
    enable = true,
  },
})

-- LSP configuration
local lspconfig = require('lspconfig')
local capabilities = require('cmp_nvim_lsp').default_capabilities()

lspconfig.infra_lsp.setup {
  capabilities = capabilities,
  cmd = { 'infra-lsp' },
  filetypes = { 'infra' },
  root_dir = lspconfig.util.root_pattern('.git'),
  settings = {
    infra = {
      lsp = {
        enabled = true,
        path = vim.fn.expand('$HOME/.local/bin/infra-lsp'),
      },
      formatting = {
        enabled = true,
      },
    },
  },
}

-- Completion setup
local cmp = require('cmp')
cmp.setup({
  sources = {
    { name = 'nvim_lsp' },
    { name = 'buffer' },
    { name = 'path' },
  },
})
```

### 3. Alternative: Using Mason.nvim

```lua
require('mason').setup()
require('mason-lspconfig').setup()

-- Automatically install infra-lsp
require('mason-lspconfig').setup({
  infra_lsp = {
    cmd = { 'infra-lsp' },
    filetypes = { 'infra' },
    root_dir = lspconfig.util.root_pattern('.git'),
    settings = {
      infra = {
        lsp = {
          enabled = true,
        },
      },
    },
  },
})
```

## Features

- **Syntax highlighting**: Full syntax highlighting for Infra code
- **LSP support**: 
  - Code completion
  - Go to definition
  - Find references
  - Hover documentation
  - Error checking
  - Code formatting
- **Snippets**: Common code patterns
- **Auto-formatting**: Automatic code formatting on save

## Troubleshooting

### Tree-sitter issues
```bash
# Reinstall the parser
nvim --head -c "TSUninstall infra"
nvim --head -c "TSInstallSync infra"
```

### LSP issues
```bash
# Check if LSP is running
:lspinfo

# Restart LSP
:LspRestart
```