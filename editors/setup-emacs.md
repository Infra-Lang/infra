# Emacs Setup for Infra Language

## Prerequisites

- Emacs 27.1 or later
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
```

### 2. Emacs Configuration

Add this to your `init.el` or `config.el`:

```elisp
;; Tree-sitter configuration
(use-package tree-sitter
  :ensure t
  :config
  (global-tree-sitter-mode)
  (add-to-list 'tree-sitter-major-mode-language-alist '(infra-mode . infra))
  (tree-sitter-require 'infra))

;; Infra mode definition
(define-derived-mode infra-mode prog-mode "Infra"
  "Major mode for Infra programming language."
  :syntax-table nil
  (setq font-lock-defaults nil)
  (setq comment-start "//")
  (setq comment-end "")
  (setq-local-comment-add-script-functions nil)
  (setq-local-comment-start-skip nil))

;; Auto-enable tree-sitter for .if files
(add-to-list 'auto-mode-alist '("\\.if\\'" . infra-mode))

;; LSP configuration
(use-package lsp-mode
  :ensure t
  :commands lsp)
  
(use-package lsp-infrafoundation
  :ensure t
  :config
  (lsp-register-client
   (make-lsp-client :new-connection
                    (lsp-stdio-connection
                     `(,(expand-file-name "~/.local/bin/infra-lsp")))
    :major-modes '(infra-mode)
    :server-id 'infra-lsp))
  (add-to-list 'lsp-language-id-configuration '(infra-mode . "infra")))

;; Company mode for completion
(use-package company
  :ensure t
  :config
  (global-company-mode))

;; Tree-sitter indent support
(use-package tree-sitter-indent
  :ensure t
  :after tree-sitter
  :config
  (add-hook 'tree-sitter-after-on-hook #'tree-sitter-indent-mode))

;; Snippet support
(use-package yasnippet
  :ensure t
  :config
  (yas-global-mode 1))
```

### 3. Tree-sitter Indent Rules

Create `tree-sitter-infra.el`:

```elisp
;;; tree-sitter-infra.el --- Infra tree-sitter support -*- lexical-binding: t; -*-

;;; Commentary:
;; Tree-sitter support for Infra language.

;;; Code:

(require 'tree-sitter)
(require 'tree-sitter-langs)

(defconst-tree-sitter-language :infra "tree_sitter_infra")

;;;### Indentation

(defvar tree-sitter-infra-indent-rules
  `((node-is-named-paren "block_statement") parent-bolp 0)
    (node-is "statement" parent-bolp tree-sitter-infra-indent-offset)
    (node-is "comment" parent-bolp 0)
    ((node-is "}") parent-bolp tree-sitter-infra-indent-offset)
    ((node-is ")") parent-bolp tree-sitter-infra-indent-offset)
    (pair "," 
     ,(and (parent-is-named-paren "parameter_list") 
            (not (parent-is-named-paren "function_declaration")))
     tree-sitter-infra-indent-offset)
    (pair ";" nil 0)))

;;;### Font-lock

(defvar tree-sitter-infra-font-lock-settings
  tree-sitter-font-lock-default-settings)

;;;### Indent support

(define-derived-mode infra-mode prog-mode "Infra"
  "Major mode for editing Infra code."
  :syntax-table nil
  :group 'tree-sitter
  :after-set syntax-table (syntax-propertize (syntax-propertize-rules infra-mode-syntax-table))
  :local-vars
  ((comment-start "// . "")
   (comment-end "")
   (comment-start-skip nil)
   (parse-sexp nil)
   (syntax-propertize-function nil)
   (treesit-font-lock-settings tree-sitter-infra-font-lock-settings)
   (treesit-parser . tree-sitter--parser)
   (treesit-language . infra)
   (indent-line-function . tree-sitter-indent)
   (indent-region-function . tree-sitter-indent-region)
   (indent-line-function . tree-sitter-indent)
   (indent-region-function . tree-sitter-indent-region))
  :keymap
  (prog-mode-map)
  (tree-sitter-attach 'infra-mode)
  (setq-local-comment-add-script-functions nil)
  (setq-local-comment-start-skip nil))

(provide 'tree-sitter-infra)
;;; tree-sitter-infra.el ends here
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
- **Auto-indentation**: Smart indentation based on syntax
- **Snippets**: Common code patterns (requires yasnippet)
- **Error checking**: Real-time syntax and type errors

## Key Bindings

```elisp
;; Custom key bindings for infra-mode
(define-key infra-mode-map (kbd "C-c C-c") #'comment-region)
(define-key infra-mode-map (kbd "C-c C-u") #'uncomment-region)
(define-key infra-mode-map (kbd "C-c C-f") #'lsp-format-buffer)
```

## Troubleshooting

### Tree-sitter issues
```elisp
;; Rebuild parser
(tree-sitter-reload-grammar 'infra)
```

### LSP issues
```elisp
;; Check LSP status
(lsp-ui-doc-show)
(lsp-ui-peek-show)
```