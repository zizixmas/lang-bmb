//! BMB Language Server Protocol implementation
//!
//! Provides IDE features:
//! - Diagnostics (type errors, parse errors)
//! - Hover (type information)
//! - Completion (keywords, built-ins)

use std::collections::HashMap;
use std::sync::RwLock;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::ast::{Program, Span};
use crate::error::CompileError;
use crate::lexer;
use crate::parser;
use crate::types::TypeChecker;

/// BMB Language keywords for completion
const BMB_KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "then", "else", "match", "for", "in", "while",
    "struct", "enum", "type", "pub", "use", "pre", "post", "where",
    "true", "false", "rec", "own", "ref", "move", "copy", "drop", "linear",
    "forall", "exists", "old", "ret", "low", "satisfies", "modifies",
    "invariant", "decreases",
];

/// BMB built-in functions for completion
const BMB_BUILTINS: &[(&str, &str)] = &[
    ("print", "print(x: i64) -> Unit"),
    ("println", "println(x: i64) -> Unit"),
    ("assert", "assert(cond: bool) -> Unit"),
    ("read_int", "read_int() -> i64"),
    ("abs", "abs(n: i64) -> i64"),
    ("min", "min(a: i64, b: i64) -> i64"),
    ("max", "max(a: i64, b: i64) -> i64"),
];

/// Document state
struct DocumentState {
    content: String,
    ast: Option<Program>,
    #[allow(dead_code)]
    version: i32,
}

/// BMB Language Server Backend
pub struct Backend {
    client: Client,
    documents: RwLock<HashMap<Url, DocumentState>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    /// Analyze document and publish diagnostics
    async fn analyze_document(&self, uri: &Url, content: &str, version: i32) {
        let diagnostics = self.get_diagnostics(uri, content);

        // Parse AST if successful for hover/completion
        let ast = self.try_parse(content);

        // Store document state
        {
            let mut docs = self.documents.write().unwrap();
            docs.insert(uri.clone(), DocumentState {
                content: content.to_string(),
                ast,
                version,
            });
        }

        // Publish diagnostics
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, Some(version))
            .await;
    }

    /// Get diagnostics from lexer, parser, and type checker
    fn get_diagnostics(&self, uri: &Url, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let filename = uri.path();

        // Try to tokenize
        let tokens = match lexer::tokenize(content) {
            Ok(tokens) => tokens,
            Err(e) => {
                diagnostics.push(self.error_to_diagnostic(&e, content));
                return diagnostics;
            }
        };

        // Try to parse
        let ast = match parser::parse(filename, content, tokens) {
            Ok(ast) => ast,
            Err(e) => {
                diagnostics.push(self.error_to_diagnostic(&e, content));
                return diagnostics;
            }
        };

        // Type check
        let mut checker = TypeChecker::new();
        if let Err(e) = checker.check_program(&ast) {
            diagnostics.push(self.error_to_diagnostic(&e, content));
        }

        diagnostics
    }

    /// Try to parse content, returning AST if successful
    fn try_parse(&self, content: &str) -> Option<Program> {
        let tokens = lexer::tokenize(content).ok()?;
        parser::parse("<lsp>", content, tokens).ok()
    }

    /// Convert CompileError to LSP Diagnostic
    fn error_to_diagnostic(&self, error: &CompileError, content: &str) -> Diagnostic {
        let (range, severity) = if let Some(span) = error.span() {
            (self.span_to_range(span, content), DiagnosticSeverity::ERROR)
        } else {
            (Range::default(), DiagnosticSeverity::ERROR)
        };

        let source = match error {
            CompileError::Lexer { .. } => "bmb-lexer",
            CompileError::Parser { .. } => "bmb-parser",
            CompileError::Type { .. } => "bmb-types",
            _ => "bmb",
        };

        Diagnostic {
            range,
            severity: Some(severity),
            source: Some(source.to_string()),
            message: error.message().to_string(),
            ..Default::default()
        }
    }

    /// Convert Span (byte offset) to LSP Range (line/character)
    fn span_to_range(&self, span: Span, content: &str) -> Range {
        let start = self.offset_to_position(span.start, content);
        let end = self.offset_to_position(span.end, content);
        Range { start, end }
    }

    /// Convert byte offset to LSP Position
    fn offset_to_position(&self, offset: usize, content: &str) -> Position {
        let mut line = 0u32;
        let mut col = 0u32;

        for (i, c) in content.char_indices() {
            if i >= offset {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        Position::new(line, col)
    }

    /// Convert LSP Position to byte offset
    fn position_to_offset(&self, position: Position, content: &str) -> usize {
        let mut current_line = 0u32;
        let mut current_col = 0u32;

        for (i, c) in content.char_indices() {
            if current_line == position.line && current_col == position.character {
                return i;
            }
            if c == '\n' {
                if current_line == position.line {
                    return i;
                }
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
        }

        content.len()
    }

    /// Get word at position for hover
    fn get_word_at_position(&self, content: &str, position: Position) -> Option<String> {
        let offset = self.position_to_offset(position, content);

        // Find word boundaries
        let bytes = content.as_bytes();
        let mut start = offset;
        let mut end = offset;

        // Walk back to find start of word
        while start > 0 && Self::is_ident_char(bytes[start - 1] as char) {
            start -= 1;
        }

        // Walk forward to find end of word
        while end < bytes.len() && Self::is_ident_char(bytes[end] as char) {
            end += 1;
        }

        if start < end {
            Some(content[start..end].to_string())
        } else {
            None
        }
    }

    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "bmb-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "BMB Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        let version = params.text_document.version;

        self.analyze_document(&uri, &content, version).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        // Full sync - take the whole content
        if let Some(change) = params.content_changes.into_iter().next() {
            self.analyze_document(&uri, &change.text, version).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut docs = self.documents.write().unwrap();
        docs.remove(&params.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().unwrap();
        let doc = match docs.get(uri) {
            Some(doc) => doc,
            None => return Ok(None),
        };

        let word = match self.get_word_at_position(&doc.content, position) {
            Some(w) => w,
            None => return Ok(None),
        };

        // Check if it's a keyword
        if BMB_KEYWORDS.contains(&word.as_str()) {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**Keyword**: `{}`", word),
                }),
                range: None,
            }));
        }

        // Check if it's a built-in function
        for (name, sig) in BMB_BUILTINS {
            if *name == word {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("**Built-in**: `{}`", sig),
                    }),
                    range: None,
                }));
            }
        }

        // Check AST for user-defined symbols
        if let Some(ast) = &doc.ast {
            for item in &ast.items {
                match item {
                    crate::ast::Item::FnDef(f) if f.name.node == word => {
                        let params: Vec<String> = f.params.iter()
                            .map(|p| format!("{}: {:?}", p.name.node, p.ty.node))
                            .collect();
                        let sig = format!("fn {}({}) -> {:?}",
                            f.name.node,
                            params.join(", "),
                            f.ret_ty.node
                        );
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!("```bmb\n{}\n```", sig),
                            }),
                            range: None,
                        }));
                    }
                    crate::ast::Item::StructDef(s) if s.name.node == word => {
                        let fields: Vec<String> = s.fields.iter()
                            .map(|f| format!("  {}: {:?}", f.name.node, f.ty.node))
                            .collect();
                        let def = format!("struct {} {{\n{}\n}}", s.name.node, fields.join(",\n"));
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!("```bmb\n{}\n```", def),
                            }),
                            range: None,
                        }));
                    }
                    crate::ast::Item::EnumDef(e) if e.name.node == word => {
                        let variants: Vec<String> = e.variants.iter()
                            .map(|v| format!("  {}", v.name.node))
                            .collect();
                        let def = format!("enum {} {{\n{}\n}}", e.name.node, variants.join(",\n"));
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: format!("```bmb\n{}\n```", def),
                            }),
                            range: None,
                        }));
                    }
                    _ => {}
                }
            }
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;

        let mut items = Vec::new();

        // Add keywords
        for keyword in BMB_KEYWORDS {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("keyword".to_string()),
                ..Default::default()
            });
        }

        // Add built-in functions
        for (name, sig) in BMB_BUILTINS {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(sig.to_string()),
                insert_text: Some(format!("{}($0)", name)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        // Add user-defined symbols from AST
        let docs = self.documents.read().unwrap();
        if let Some(doc) = docs.get(uri) {
            if let Some(ast) = &doc.ast {
                for item in &ast.items {
                    match item {
                        crate::ast::Item::FnDef(f) => {
                            let params: Vec<String> = f.params.iter()
                                .enumerate()
                                .map(|(i, p)| format!("${{{}:{}}}", i + 1, p.name.node))
                                .collect();
                            items.push(CompletionItem {
                                label: f.name.node.clone(),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some(format!("fn -> {:?}", f.ret_ty.node)),
                                insert_text: Some(format!("{}({})", f.name.node, params.join(", "))),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                        crate::ast::Item::StructDef(s) => {
                            items.push(CompletionItem {
                                label: s.name.node.clone(),
                                kind: Some(CompletionItemKind::STRUCT),
                                detail: Some("struct".to_string()),
                                ..Default::default()
                            });
                        }
                        crate::ast::Item::EnumDef(e) => {
                            items.push(CompletionItem {
                                label: e.name.node.clone(),
                                kind: Some(CompletionItemKind::ENUM),
                                detail: Some("enum".to_string()),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }
}

/// Start the LSP server
pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
