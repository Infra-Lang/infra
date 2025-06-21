// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import * as path from 'path';
import { exec } from 'child_process';
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "infra-lang-support" is now active!');

	// Start the Language Server
	startLanguageServer(context);

	// Register command to run Infra files
	const runFileCommand = vscode.commands.registerCommand('infra-lang-support.runFile', () => {
		const activeEditor = vscode.window.activeTextEditor;
		if (!activeEditor) {
			vscode.window.showErrorMessage('No active file to run');
			return;
		}

		const document = activeEditor.document;
		if (document.languageId !== 'infra') {
			vscode.window.showErrorMessage('This command can only be used with Infra files (.if or .infra)');
			return;
		}

		// Save the file before running
		document.save().then(() => {
			runInfraFile(document.uri.fsPath);
		});
	});

	// Register command to check syntax
	const checkSyntaxCommand = vscode.commands.registerCommand('infra-lang-support.checkSyntax', () => {
		const activeEditor = vscode.window.activeTextEditor;
		if (!activeEditor) {
			vscode.window.showErrorMessage('No active file to check');
			return;
		}

		const document = activeEditor.document;
		if (document.languageId !== 'infra') {
			vscode.window.showErrorMessage('This command can only be used with Infra files (.if or .infra)');
			return;
		}

		// Save the file before checking
		document.save().then(() => {
			checkInfraSyntax(document.uri.fsPath);
		});
	});

	// Register diagnostic provider for real-time error checking
	const diagnosticCollection = vscode.languages.createDiagnosticCollection('infra');
	context.subscriptions.push(diagnosticCollection);

	// Setup document change listener for real-time diagnostics
	const documentChangeListener = vscode.workspace.onDidSaveTextDocument((document) => {
		if (document.languageId === 'infra') {
			updateDiagnostics(document, diagnosticCollection);
		}
	});

	// Setup document open listener
	const documentOpenListener = vscode.workspace.onDidOpenTextDocument((document) => {
		if (document.languageId === 'infra') {
			updateDiagnostics(document, diagnosticCollection);
		}
	});

	// Check diagnostics for already open documents
	vscode.workspace.textDocuments.forEach(document => {
		if (document.languageId === 'infra') {
			updateDiagnostics(document, diagnosticCollection);
		}
	});

	context.subscriptions.push(
		runFileCommand,
		checkSyntaxCommand,
		documentChangeListener,
		documentOpenListener
	);

	// Register IntelliSense providers
	context.subscriptions.push(
		// Completion provider for IntelliSense
		vscode.languages.registerCompletionItemProvider('infra', new InfraCompletionProvider(), '.', ' '),
		
		// Hover provider for documentation
		vscode.languages.registerHoverProvider('infra', new InfraHoverProvider()),
		
		// Document symbol provider for outline
		vscode.languages.registerDocumentSymbolProvider('infra', new InfraDocumentSymbolProvider()),
		
		// Definition provider for go-to-definition
		vscode.languages.registerDefinitionProvider('infra', new InfraDefinitionProvider())
	);
}

// Function to run Infra file
function runInfraFile(filePath: string) {
	const terminal = vscode.window.createTerminal('Infra Runner');
	terminal.show();
	
	// Find the Infra interpreter (assuming it's in the parent directory)
	const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
	if (!workspaceRoot) {
		vscode.window.showErrorMessage('No workspace folder found');
		return;
	}

	const infraRoot = path.join(workspaceRoot, '..');
	const relativePath = path.relative(infraRoot, filePath);
	
	terminal.sendText(`cd "${infraRoot}"`);
	terminal.sendText(`cargo run -- "${relativePath}"`);
}

// Function to check Infra syntax
function checkInfraSyntax(filePath: string) {
	const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
	if (!workspaceRoot) {
		vscode.window.showErrorMessage('No workspace folder found');
		return;
	}

	const infraRoot = path.join(workspaceRoot, '..');
	const relativePath = path.relative(infraRoot, filePath);
	
	exec(`cd "${infraRoot}" && cargo run -- "${relativePath}"`, (error, stdout, stderr) => {
		if (error) {
			vscode.window.showErrorMessage(`Syntax Error: ${stderr || error.message}`);
		} else {
			vscode.window.showInformationMessage('Syntax check passed!');
		}
	});
}

// Function to update diagnostics
function updateDiagnostics(document: vscode.TextDocument, collection: vscode.DiagnosticCollection) {
	const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
	if (!workspaceRoot) {
		return;
	}

	const infraRoot = path.join(workspaceRoot, '..');
	const relativePath = path.relative(infraRoot, document.uri.fsPath);
	
	exec(`cd "${infraRoot}" && cargo run -- "${relativePath}"`, (error, stdout, stderr) => {
		const diagnostics: vscode.Diagnostic[] = [];
		
		if (error && stderr) {
			// Parse error messages from Infra interpreter
			const errorLines = stderr.split('\n');
			for (const line of errorLines) {
				const match = line.match(/\[line (\d+), column (\d+)\]\s*(.+)/);
				if (match) {
					const lineNum = parseInt(match[1]) - 1; // VS Code uses 0-based line numbers
					const colNum = parseInt(match[2]) - 1;  // VS Code uses 0-based column numbers
					const message = match[3];
					
					const range = new vscode.Range(
						new vscode.Position(lineNum, colNum),
						new vscode.Position(lineNum, colNum + 10) // Approximate error range
					);
					
					const diagnostic = new vscode.Diagnostic(
						range,
						message,
						vscode.DiagnosticSeverity.Error
					);
					
					diagnostics.push(diagnostic);
				}
			}
		}
		
		collection.set(document.uri, diagnostics);
	});
}

// IntelliSense completion provider for Infra language
class InfraCompletionProvider implements vscode.CompletionItemProvider {
	
	provideCompletionItems(document: vscode.TextDocument, position: vscode.Position): vscode.CompletionItem[] {
		const completions: vscode.CompletionItem[] = [];
		
		// Keywords
		const keywords = [
			'let', 'function', 'if', 'else', 'for', 'while', 'try', 'catch', 
			'return', 'import', 'export', 'from', 'true', 'false', 'null'
		];
		
		keywords.forEach(keyword => {
			const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
			item.detail = `Infra keyword: ${keyword}`;
			completions.push(item);
		});
		
		// Type annotations
		const types = ['number', 'string', 'boolean', 'array', 'object'];
		types.forEach(type => {
			const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.TypeParameter);
			item.detail = `Infra type: ${type}`;
			completions.push(item);
		});
		
		// Built-in functions - Math module
		const mathFunctions = [
			{ name: 'math.sqrt', detail: 'math.sqrt(number) -> number', doc: 'Calculate square root' },
			{ name: 'math.pow', detail: 'math.pow(base, exponent) -> number', doc: 'Calculate power' },
			{ name: 'math.abs', detail: 'math.abs(number) -> number', doc: 'Absolute value' },
			{ name: 'math.floor', detail: 'math.floor(number) -> number', doc: 'Round down to integer' },
			{ name: 'math.ceil', detail: 'math.ceil(number) -> number', doc: 'Round up to integer' }
		];
		
		mathFunctions.forEach(func => {
			const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
			item.detail = func.detail;
			item.documentation = new vscode.MarkdownString(func.doc);
			item.insertText = new vscode.SnippetString(`${func.name.split('.')[1]}($1)`);
			completions.push(item);
		});
		
		// Built-in functions - String module
		const stringFunctions = [
			{ name: 'string.length', detail: 'string.length(str) -> number', doc: 'Get string length' },
			{ name: 'string.upper', detail: 'string.upper(str) -> string', doc: 'Convert to uppercase' },
			{ name: 'string.lower', detail: 'string.lower(str) -> string', doc: 'Convert to lowercase' },
			{ name: 'string.substring', detail: 'string.substring(str, start, end) -> string', doc: 'Extract substring' }
		];
		
		stringFunctions.forEach(func => {
			const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
			item.detail = func.detail;
			item.documentation = new vscode.MarkdownString(func.doc);
			item.insertText = new vscode.SnippetString(`${func.name.split('.')[1]}($1)`);
			completions.push(item);
		});
		
		// Built-in functions - Array module
		const arrayFunctions = [
			{ name: 'array.length', detail: 'array.length(arr) -> number', doc: 'Get array length' },
			{ name: 'array.push', detail: 'array.push(arr, value) -> array', doc: 'Add element to end' },
			{ name: 'array.pop', detail: 'array.pop(arr) -> value', doc: 'Remove last element' },
			{ name: 'array.sort', detail: 'array.sort(arr) -> array', doc: 'Sort array' },
			{ name: 'array.reverse', detail: 'array.reverse(arr) -> array', doc: 'Reverse array' }
		];
		
		arrayFunctions.forEach(func => {
			const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
			item.detail = func.detail;
			item.documentation = new vscode.MarkdownString(func.doc);
			item.insertText = new vscode.SnippetString(`${func.name.split('.')[1]}($1)`);
			completions.push(item);
		});
		
		// Built-in functions - I/O module
		const ioFunctions = [
			{ name: 'io.read_file', detail: 'io.read_file(path) -> string', doc: 'Read file contents' },
			{ name: 'io.write_file', detail: 'io.write_file(path, content) -> boolean', doc: 'Write to file' },
			{ name: 'io.exists', detail: 'io.exists(path) -> boolean', doc: 'Check if file exists' }
		];
		
		ioFunctions.forEach(func => {
			const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
			item.detail = func.detail;
			item.documentation = new vscode.MarkdownString(func.doc);
			item.insertText = new vscode.SnippetString(`${func.name.split('.')[1]}($1)`);
			completions.push(item);
		});
		
		// Common patterns and snippets
		const patterns = [
			{
				label: 'function',
				insertText: 'function ${1:name}(${2:params}):\n    ${3:// function body}\n    return ${4:value}',
				detail: 'Function definition',
				kind: vscode.CompletionItemKind.Snippet
			},
			{
				label: 'if-else',
				insertText: 'if ${1:condition}:\n    ${2:// if body}\nelse:\n    ${3:// else body}',
				detail: 'If-else statement',
				kind: vscode.CompletionItemKind.Snippet
			},
			{
				label: 'for-loop',
				insertText: 'for ${1:item} in ${2:iterable}:\n    ${3:// loop body}',
				detail: 'For loop',
				kind: vscode.CompletionItemKind.Snippet
			},
			{
				label: 'try-catch',
				insertText: 'try:\n    ${1:// try body}\ncatch ${2:error}:\n    ${3:// error handling}',
				detail: 'Try-catch block',
				kind: vscode.CompletionItemKind.Snippet
			}
		];
		
		patterns.forEach(pattern => {
			const item = new vscode.CompletionItem(pattern.label, pattern.kind);
			item.insertText = new vscode.SnippetString(pattern.insertText);
			item.detail = pattern.detail;
			completions.push(item);
		});
		
		return completions;
	}
}

// Hover provider for better documentation
class InfraHoverProvider implements vscode.HoverProvider {
	
	provideHover(document: vscode.TextDocument, position: vscode.Position): vscode.Hover | undefined {
		const range = document.getWordRangeAtPosition(position);
		const word = document.getText(range);
		
		// Documentation for keywords
		const keywordDocs: { [key: string]: string } = {
			'let': '**let** - Declares a variable\n\nExample: `let x = 42`',
			'function': '**function** - Defines a function\n\nExample: `function add(a, b): return a + b`',
			'if': '**if** - Conditional statement\n\nExample: `if condition: do_something()`',
			'for': '**for** - Loop statement\n\nExample: `for item in array: print(item)`',
			'try': '**try** - Error handling\n\nExample: `try: risky_code() catch error: handle_error()`'
		};
		
		// Documentation for built-in functions
		const functionDocs: { [key: string]: string } = {
			'print': '**print(value)** - Print value to console\n\nPrints the given value to the standard output.',
			'range': '**range(start, end)** - Generate range of numbers\n\nCreates an array of numbers from start to end (exclusive).'
		};
		
		if (keywordDocs[word]) {
			return new vscode.Hover(new vscode.MarkdownString(keywordDocs[word]));
		}
		
		if (functionDocs[word]) {
			return new vscode.Hover(new vscode.MarkdownString(functionDocs[word]));
		}
		
		return undefined;
	}
}

// Document symbol provider for outline view
class InfraDocumentSymbolProvider implements vscode.DocumentSymbolProvider {
	
	provideDocumentSymbols(document: vscode.TextDocument): vscode.DocumentSymbol[] {
		const symbols: vscode.DocumentSymbol[] = [];
		const text = document.getText();
		const lines = text.split('\n');
		
		for (let i = 0; i < lines.length; i++) {
			const line = lines[i];
			
			// Find function definitions
			const functionMatch = line.match(/^\s*function\s+(\w+)\s*\(/);
			if (functionMatch) {
				const functionName = functionMatch[1];
				const range = new vscode.Range(i, 0, i, line.length);
				const selectionRange = new vscode.Range(i, functionMatch.index! + functionMatch[0].indexOf(functionName), i, functionMatch.index! + functionMatch[0].indexOf(functionName) + functionName.length);
				
				const symbol = new vscode.DocumentSymbol(
					functionName,
					'Function',
					vscode.SymbolKind.Function,
					range,
					selectionRange
				);
				symbols.push(symbol);
			}
			
			// Find variable declarations
			const variableMatch = line.match(/^\s*let\s+(\w+)/);
			if (variableMatch) {
				const variableName = variableMatch[1];
				const range = new vscode.Range(i, 0, i, line.length);
				const selectionRange = new vscode.Range(i, variableMatch.index! + variableMatch[0].indexOf(variableName), i, variableMatch.index! + variableMatch[0].indexOf(variableName) + variableName.length);
				
				const symbol = new vscode.DocumentSymbol(
					variableName,
					'Variable',
					vscode.SymbolKind.Variable,
					range,
					selectionRange
				);
				symbols.push(symbol);
			}
		}
		
		return symbols;
	}
}

// Definition provider for go-to-definition
class InfraDefinitionProvider implements vscode.DefinitionProvider {
	
	provideDefinition(document: vscode.TextDocument, position: vscode.Position): vscode.Definition | undefined {
		const range = document.getWordRangeAtPosition(position);
		const word = document.getText(range);
		
		if (!word) {
			return undefined;
		}
		
		// Find function definitions in the current document
		const text = document.getText();
		const lines = text.split('\n');
		
		for (let i = 0; i < lines.length; i++) {
			const line = lines[i];
			const functionMatch = line.match(/^\s*function\s+(\w+)\s*\(/);
			
			if (functionMatch && functionMatch[1] === word) {
				const startPos = new vscode.Position(i, functionMatch.index! + functionMatch[0].indexOf(word));
				const endPos = new vscode.Position(i, functionMatch.index! + functionMatch[0].indexOf(word) + word.length);
				return new vscode.Location(document.uri, new vscode.Range(startPos, endPos));
			}
		}
		
		return undefined;
	}
}

// Function to start the Language Server
function startLanguageServer(context: vscode.ExtensionContext) {
	// Path to the language server
	const serverModule = context.asAbsolutePath(
		path.join('..', 'infra-language-server', 'out', 'server.js')
	);
	
	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run: { module: serverModule, transport: TransportKind.ipc },
		debug: {
			module: serverModule,
			transport: TransportKind.ipc,
		}
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for Infra documents
		documentSelector: [{ scheme: 'file', language: 'infra' }],
		synchronize: {
			// Notify the server about file changes to '.if' and '.infra' files contained in the workspace
			fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{if,infra}')
		}
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'infraLanguageServer',
		'Infra Language Server',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
}

// This method is called when your extension is deactivated
export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
