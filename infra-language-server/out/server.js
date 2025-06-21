"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("vscode-languageserver/node");
const vscode_languageserver_textdocument_1 = require("vscode-languageserver-textdocument");
// Create a connection for the server
const connection = (0, node_1.createConnection)(node_1.ProposedFeatures.all);
// Create a simple text document manager
const documents = new node_1.TextDocuments(vscode_languageserver_textdocument_1.TextDocument);
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;
connection.onInitialize((params) => {
    const capabilities = params.capabilities;
    hasConfigurationCapability = !!(capabilities.workspace && !!capabilities.workspace.configuration);
    hasWorkspaceFolderCapability = !!(capabilities.workspace && !!capabilities.workspace.workspaceFolders);
    hasDiagnosticRelatedInformationCapability = !!(capabilities.textDocument &&
        capabilities.textDocument.publishDiagnostics &&
        capabilities.textDocument.publishDiagnostics.relatedInformation);
    const result = {
        capabilities: {
            textDocumentSync: node_1.TextDocumentSyncKind.Incremental,
            // Tell the client that this server supports code completion
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['.', ' ']
            },
            // Support hover
            hoverProvider: true,
            // Support go-to-definition
            definitionProvider: true,
            // Support document symbols
            documentSymbolProvider: true
        }
    };
    if (hasWorkspaceFolderCapability) {
        result.capabilities.workspace = {
            workspaceFolders: {
                supported: true
            }
        };
    }
    return result;
});
connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        connection.client.register(node_1.DidChangeConfigurationNotification.type, undefined);
    }
    if (hasWorkspaceFolderCapability) {
        connection.workspace.onDidChangeWorkspaceFolders((_event) => {
            connection.console.log('Workspace folder change event received.');
        });
    }
});
// Global settings for all open documents
const defaultSettings = { maxNumberOfProblems: 1000 };
let globalSettings = defaultSettings;
// Cache the settings of all open documents
const documentSettings = new Map();
connection.onDidChangeConfiguration((change) => {
    if (hasConfigurationCapability) {
        documentSettings.clear();
    }
    else {
        globalSettings = ((change.settings.infraLanguageServer || defaultSettings));
    }
    // Revalidate all open text documents
    documents.all().forEach(validateTextDocument);
});
function getDocumentSettings(resource) {
    if (!hasConfigurationCapability) {
        return Promise.resolve(globalSettings);
    }
    let result = documentSettings.get(resource);
    if (!result) {
        result = connection.workspace.getConfiguration({
            scopeUri: resource,
            section: 'infraLanguageServer'
        });
        documentSettings.set(resource, result);
    }
    return result;
}
// Only keep settings for open documents
documents.onDidClose((e) => {
    documentSettings.delete(e.document.uri);
});
// The content of a text document has changed
documents.onDidChangeContent((change) => {
    validateTextDocument(change.document);
});
async function validateTextDocument(textDocument) {
    const settings = await getDocumentSettings(textDocument.uri);
    const text = textDocument.getText();
    const diagnostics = [];
    // Simple syntax validation
    const lines = text.split('\n');
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        // Check for basic syntax errors
        if (line.includes('function') && !line.includes(':')) {
            const diagnostic = {
                severity: node_1.DiagnosticSeverity.Error,
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                message: 'Function declaration must end with ":"',
                source: 'infra'
            };
            diagnostics.push(diagnostic);
        }
        // Check for let without assignment
        if (line.trim().startsWith('let ') && !line.includes('=')) {
            const diagnostic = {
                severity: node_1.DiagnosticSeverity.Error,
                range: {
                    start: { line: i, character: 0 },
                    end: { line: i, character: line.length }
                },
                message: 'Variable declaration must include assignment',
                source: 'infra'
            };
            diagnostics.push(diagnostic);
        }
        // Check for type annotation syntax
        const typeMatch = line.match(/let\s+\w+\s*:\s*(\w+)/);
        if (typeMatch) {
            const type = typeMatch[1];
            const validTypes = ['number', 'string', 'boolean', 'array', 'object'];
            if (!validTypes.includes(type)) {
                const diagnostic = {
                    severity: node_1.DiagnosticSeverity.Warning,
                    range: {
                        start: { line: i, character: line.indexOf(type) },
                        end: { line: i, character: line.indexOf(type) + type.length }
                    },
                    message: `Unknown type '${type}'. Valid types: ${validTypes.join(', ')}`,
                    source: 'infra'
                };
                diagnostics.push(diagnostic);
            }
        }
    }
    // Send the computed diagnostics to VS Code
    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}
// Completion provider
connection.onCompletion((_textDocumentPosition) => {
    const completions = [];
    // Keywords
    const keywords = [
        'let', 'function', 'if', 'else', 'for', 'while', 'try', 'catch',
        'return', 'import', 'export', 'from', 'true', 'false', 'null'
    ];
    keywords.forEach(keyword => {
        completions.push({
            label: keyword,
            kind: node_1.CompletionItemKind.Keyword,
            detail: `Infra keyword: ${keyword}`,
            documentation: getKeywordDocumentation(keyword)
        });
    });
    // Types
    const types = ['number', 'string', 'boolean', 'array', 'object'];
    types.forEach(type => {
        completions.push({
            label: type,
            kind: node_1.CompletionItemKind.TypeParameter,
            detail: `Infra type: ${type}`,
            documentation: `Type annotation for ${type} values`
        });
    });
    // Built-in functions - Math module
    const mathFunctions = [
        { name: 'math.sqrt', insertText: 'sqrt($1)', detail: 'sqrt(number) -> number', doc: 'Calculate square root' },
        { name: 'math.pow', insertText: 'pow($1, $2)', detail: 'pow(base, exponent) -> number', doc: 'Calculate power' },
        { name: 'math.abs', insertText: 'abs($1)', detail: 'abs(number) -> number', doc: 'Absolute value' },
        { name: 'math.floor', insertText: 'floor($1)', detail: 'floor(number) -> number', doc: 'Round down' },
        { name: 'math.ceil', insertText: 'ceil($1)', detail: 'ceil(number) -> number', doc: 'Round up' }
    ];
    mathFunctions.forEach(func => {
        completions.push({
            label: func.name,
            kind: node_1.CompletionItemKind.Function,
            detail: func.detail,
            documentation: func.doc,
            insertText: func.insertText,
            insertTextFormat: 2 // Snippet format
        });
    });
    // Built-in functions - String module
    const stringFunctions = [
        { name: 'string.length', insertText: 'length($1)', detail: 'length(str) -> number', doc: 'Get string length' },
        { name: 'string.upper', insertText: 'upper($1)', detail: 'upper(str) -> string', doc: 'Convert to uppercase' },
        { name: 'string.lower', insertText: 'lower($1)', detail: 'lower(str) -> string', doc: 'Convert to lowercase' },
        { name: 'string.substring', insertText: 'substring($1, $2, $3)', detail: 'substring(str, start, end) -> string', doc: 'Extract substring' }
    ];
    stringFunctions.forEach(func => {
        completions.push({
            label: func.name,
            kind: node_1.CompletionItemKind.Function,
            detail: func.detail,
            documentation: func.doc,
            insertText: func.insertText,
            insertTextFormat: 2
        });
    });
    // Built-in functions - Array module
    const arrayFunctions = [
        { name: 'array.length', insertText: 'length($1)', detail: 'length(arr) -> number', doc: 'Get array length' },
        { name: 'array.push', insertText: 'push($1, $2)', detail: 'push(arr, value) -> array', doc: 'Add element' },
        { name: 'array.pop', insertText: 'pop($1)', detail: 'pop(arr) -> value', doc: 'Remove last element' }
    ];
    arrayFunctions.forEach(func => {
        completions.push({
            label: func.name,
            kind: node_1.CompletionItemKind.Function,
            detail: func.detail,
            documentation: func.doc,
            insertText: func.insertText,
            insertTextFormat: 2
        });
    });
    return completions;
});
// Hover provider
connection.onHover((params) => {
    const document = documents.get(params.textDocument.uri);
    if (!document)
        return undefined;
    const position = params.position;
    const text = document.getText();
    const lines = text.split('\n');
    const line = lines[position.line];
    // Get word at position
    const wordMatch = line.match(/\b\w+\b/g);
    if (!wordMatch)
        return undefined;
    let currentPos = 0;
    for (const word of wordMatch) {
        const wordStart = line.indexOf(word, currentPos);
        const wordEnd = wordStart + word.length;
        if (position.character >= wordStart && position.character <= wordEnd) {
            const documentation = getKeywordDocumentation(word);
            if (documentation) {
                return {
                    contents: {
                        kind: node_1.MarkupKind.Markdown,
                        value: documentation
                    }
                };
            }
        }
        currentPos = wordEnd;
    }
    return undefined;
});
// Definition provider
connection.onDefinition((params) => {
    const document = documents.get(params.textDocument.uri);
    if (!document)
        return undefined;
    const position = params.position;
    const text = document.getText();
    const lines = text.split('\n');
    const line = lines[position.line];
    // Get word at position
    const wordMatch = line.match(/\b\w+\b/g);
    if (!wordMatch)
        return undefined;
    let currentPos = 0;
    for (const word of wordMatch) {
        const wordStart = line.indexOf(word, currentPos);
        const wordEnd = wordStart + word.length;
        if (position.character >= wordStart && position.character <= wordEnd) {
            // Find function definition
            for (let i = 0; i < lines.length; i++) {
                const functionMatch = lines[i].match(new RegExp(`function\\s+${word}\\s*\\(`));
                if (functionMatch) {
                    return node_1.Location.create(params.textDocument.uri, node_1.Range.create(i, functionMatch.index, i, functionMatch.index + functionMatch[0].length));
                }
            }
        }
        currentPos = wordEnd;
    }
    return undefined;
});
// Document symbols provider
connection.onDocumentSymbol((params) => {
    const document = documents.get(params.textDocument.uri);
    if (!document)
        return [];
    const symbols = [];
    const text = document.getText();
    const lines = text.split('\n');
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        // Find function definitions
        const functionMatch = line.match(/function\s+(\w+)\s*\(/);
        if (functionMatch) {
            const functionName = functionMatch[1];
            const range = node_1.Range.create(i, 0, i, line.length);
            const selectionRange = node_1.Range.create(i, functionMatch.index + functionMatch[0].indexOf(functionName), i, functionMatch.index + functionMatch[0].indexOf(functionName) + functionName.length);
            symbols.push({
                name: functionName,
                detail: 'Function',
                kind: node_1.SymbolKind.Function,
                range,
                selectionRange
            });
        }
        // Find variable declarations
        const variableMatch = line.match(/let\s+(\w+)/);
        if (variableMatch) {
            const variableName = variableMatch[1];
            const range = node_1.Range.create(i, 0, i, line.length);
            const selectionRange = node_1.Range.create(i, variableMatch.index + variableMatch[0].indexOf(variableName), i, variableMatch.index + variableMatch[0].indexOf(variableName) + variableName.length);
            symbols.push({
                name: variableName,
                detail: 'Variable',
                kind: node_1.SymbolKind.Variable,
                range,
                selectionRange
            });
        }
    }
    return symbols;
});
function getKeywordDocumentation(keyword) {
    const docs = {
        'let': '**let** - Declares a variable\n\nExample: `let x = 42`',
        'function': '**function** - Defines a function\n\nExample: `function add(a, b): return a + b`',
        'if': '**if** - Conditional statement\n\nExample: `if condition: do_something()`',
        'for': '**for** - Loop statement\n\nExample: `for item in array: print(item)`',
        'while': '**while** - While loop\n\nExample: `while condition: continue()`',
        'try': '**try** - Error handling\n\nExample: `try: risky_code() catch error: handle_error()`',
        'return': '**return** - Return from function\n\nExample: `return value`',
        'import': '**import** - Import from module\n\nExample: `import {func} from "module"`',
        'export': '**export** - Export declaration\n\nExample: `export function func(): {}`',
        'print': '**print(value)** - Print value to console\n\nPrints the given value to the standard output.',
        'range': '**range(start, end)** - Generate range of numbers\n\nCreates an array of numbers from start to end (exclusive).'
    };
    return docs[keyword];
}
// Make the text document manager listen on the connection
documents.listen(connection);
// Listen on the connection
connection.listen();
//# sourceMappingURL=server.js.map