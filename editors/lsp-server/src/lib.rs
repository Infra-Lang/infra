use async_trait::async_trait;
use dashmap::DashMap;
use log::{debug, error, info, warn};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tower_lsp::{
    jsonrpc::{Error, Result},
    lsp_types::{
        self, *,
    },
    Client, LanguageServer,
};

pub struct Server {
    client: Arc<Client>,
    documents: Arc<DashMap<lsp_types::Url, lsp_types::TextDocumentItem>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Client::new()),
            documents: Arc::new(DashMap::new()),
        }
    }

    fn uri_to_path(uri: &lsp_types::Url) -> Option<String> {
        uri.to_file_path()
            .ok()
            .and_then(|path| path.to_str().map(|s| s.to_string()))
    }
}

#[async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("Initializing LSP server for Infra");

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::Incremental,
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![
                    ".".to_string(),
                    "(".to_string(),
                    " ".to_string(),
                    "\"".to_string(),
                    "'".to_string(),
                    "/".to_string(),
                ]),
                work_done_progress_options: Default::default(),
                all_commit_characters: None,
            }),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            document_highlight_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            code_action_provider: Some(CodeActionProviderCapability::Simple(
                CodeActionOptions {
                    code_action_kinds: Some(vec![
                        CodeActionKind::QUICKFIX,
                        CodeActionKind::REFACTOR,
                    ]),
                    resolve_provider: Some(false),
                    work_done_progress_options: Default::default(),
                },
            )),
            code_lens_provider: None,
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: Some(vec![
                    "(".to_string(),
                    ",".to_string(),
                ]),
                retrigger_characters: None,
                work_done_progress_options: Default::default(),
            }),
            rename_provider: Some(OneOf::Left(true)),
            prepare_rename_provider: None,
            execute_command_provider: None,
            workspace: None,
            semantic_tokens_provider: None,
            moniker_provider: None,
            linked_editing_range_provider: None,
            call_hierarchy_provider: None,
            type_definition_provider: None,
            implementation_provider: None,
            color_provider: None,
            folding_range_provider: None,
            selection_range_provider: None,
            declaration_provider: None,
            workspace_folders_provider: None,
        };

        Ok(InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "Infra Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("LSP server initialized");
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down LSP server");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        let uri = text_document.uri.clone();

        self.documents.insert(uri, text_document);
        info!("Opened document: {}", uri);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        if let Some(doc) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                match change {
                    TextDocumentContentChangeEvent::Full { text } => {
                        doc.text = text;
                    }
                    TextDocumentContentChangeEvent::Incremental { range, text: change_text } => {
                        // For simplicity, we'll handle full text changes only for now
                        // In a full implementation, you'd apply incremental changes
                        warn!("Incremental changes not fully implemented");
                    }
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.remove(&uri);
        info!("Closed document: {}", uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<Vec<CompletionItem>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        }

        let text = &doc.text;
        let lines: Vec<&str> = text.lines().collect();

        if position.line >= lines.len() {
            return Ok(None);
        }

        let current_line = lines[position.line];
        let line_prefix = &current_line[..position.character.min(current_line.len())];

        let mut completions = Vec::new();

        // Basic keyword completions
        let keywords = vec![
            "function", "class", "let", "return", "if", "else", "for", "while",
            "break", "continue", "import", "export", "and", "or", "not", "in",
            "await", "async", "true", "false", "nil"
        ];

        for keyword in keywords {
            if keyword.starts_with(line_prefix) {
                completions.push(CompletionItem {
                    label: keyword.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("Infra keyword".to_string()),
                    documentation: Some(Documentation::String(format!("Infra keyword: {}", keyword))),
                    ..Default::default()
                });
            }
        }

        // Function completions
        let functions = vec![
            "print", "len", "abs", "max", "min", "round", "floor", "ceil",
            "type", "str", "int", "float", "bool", "array", "object",
        ];

        for function in functions {
            if function.starts_with(line_prefix) {
                completions.push(CompletionItem {
                    label: function.to_string(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some("Built-in function".to_string()),
                    documentation: Some(Documentation::String(format!("Built-in function: {}", function))),
                    ..Default::default()
                });
            }
        }

        Ok(Some(completions))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        }

        let text = &doc.text;
        let lines: Vec<&str> = text.lines().collect();

        if position.line >= lines.len() {
            return Ok(None);
        }

        let current_line = lines[position.line];
        let word_at_position = self.get_word_at_position(current_line, position.character);

        if let Some(word) = word_at_position {
            let content = match word.as_str() {
                "function" => "Defines a reusable function in Infra.\n\nExample:\nfunction add(a, b): number {\n  return a + b\n}",
                "class" => "Defines a class for object-oriented programming.\n\nExample:\nclass Person:\n  function init(name):\n    this.name = name",
                "let" => "Declares a variable with optional type annotation.\n\nExample:\nlet x: number = 42",
                "if" => "Conditional statement.\n\nExample:\nif x > 0:\n  print(\"positive\")",
                "for" => "Loop over iterables.\n\nExample:\nfor item in array:\n  print(item)",
                "while" => "Conditional loop.\n\nExample:\nwhile condition:\n  do_something()",
                "return" => "Returns a value from a function.\n\nExample:\nreturn result",
                _ => return Ok(None),
            };

            return Ok(Some(Hover {
                contents: HoverContents::Markdown(content.to_string()),
                range: Some(Range {
                    start: Position {
                        line: position.line,
                        character: position.character.saturating_sub(word.len()),
                    },
                    end: Position {
                        line: position.line,
                        character: position.character,
                    },
                }),
            }));
        }

        Ok(None)
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // Basic implementation - would need full parser for real definitions
        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        // Basic implementation - would need full parser for real references
        Ok(None)
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<Vec<DocumentSymbol>>> {
        // Basic implementation - would need full parser for real symbols
        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;

        let Some(doc) = self.documents.get(&uri) else {
            return Ok(None);
        }

        let formatted = self.format_code(&doc.text, &params.options);

        let edit = TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(u32::MAX, u32::MAX),
            },
            new_text: formatted,
        };

        Ok(Some(vec![edit]))
    }
}

impl Server {
    fn get_word_at_position(&self, line: &str, character: u32) -> Option<String> {
        let char_idx = character as usize;
        if char_idx >= line.len() {
            return None;
        }

        // Find word boundaries around cursor
        let mut start = char_idx;
        let mut end = char_idx;

        // Find start of word
        while start > 0 {
            let ch = line.chars().nth(start - 1)?;
            if ch.is_alphanumeric() || ch == '_' {
                start -= 1;
            } else {
                break;
            }
        }

        // Find end of word
        while end < line.len() {
            let ch = line.chars().nth(end)?;
            if ch.is_alphanumeric() || ch == '_' {
                end += 1;
            } else {
                break;
            }
        }

        if start < end {
            Some(line[start..end].to_string())
        } else {
            None
        }
    }

    fn format_code(&self, code: &str, _options: &FormattingOptions) -> String {
        // Basic formatting - would need full parser for proper formatting
        // For now, just return the original code
        code.to_string()
    }
}
