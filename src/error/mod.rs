use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone)]
pub enum FeraError {
    LexError(LexError),
    ParseError(ParseError),
    TypeError(TypeError),
    CodegenError(CodegenError),
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub span: Span,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub span: Span,
    pub expected_type: Option<String>,
    pub found_type: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CodegenError {
    pub message: String,
    pub span: Option<Span>,
    pub suggestion: Option<String>,
}

impl FeraError {
    pub fn report(&self, filename: &str, source: &str) {
        let mut files = SimpleFiles::new();
        let file_id = files.add(filename, source);
        
        let diagnostic = match self {
            FeraError::LexError(e) => {
                let mut diag = Diagnostic::error()
                    .with_message(format!("Lexer error: {}", e.message))
                    .with_labels(vec![
                        Label::primary(file_id, e.span.clone())
                            .with_message(&e.message)
                    ]);
                
                if let Some(suggestion) = &e.suggestion {
                    diag = diag.with_notes(vec![format!("help: {}", suggestion)]);
                }
                
                diag
            }
            FeraError::ParseError(e) => {
                let mut message = e.message.clone();
                if let (Some(expected), Some(found)) = (&e.expected, &e.found) {
                    message = format!("expected {}, found {}", expected, found);
                }
                
                let mut diag = Diagnostic::error()
                    .with_message(format!("Parse error: {}", message))
                    .with_labels(vec![
                        Label::primary(file_id, e.span.clone())
                            .with_message(&message)
                    ]);
                
                if let Some(suggestion) = &e.suggestion {
                    diag = diag.with_notes(vec![format!("help: {}", suggestion)]);
                }
                
                diag
            }
            FeraError::TypeError(e) => {
                let mut message = e.message.clone();
                if let (Some(expected), Some(found)) = (&e.expected_type, &e.found_type) {
                    message = format!("type mismatch: expected {}, found {}", expected, found);
                }
                
                let mut diag = Diagnostic::error()
                    .with_message(format!("Type error: {}", message))
                    .with_labels(vec![
                        Label::primary(file_id, e.span.clone())
                            .with_message(&message)
                    ]);
                
                if let Some(suggestion) = &e.suggestion {
                    diag = diag.with_notes(vec![format!("help: {}", suggestion)]);
                }
                
                diag
            }
            FeraError::CodegenError(e) => {
                let mut diag = Diagnostic::error()
                    .with_message(format!("Codegen error: {}", e.message));
                
                if let Some(span) = &e.span {
                    diag = diag.with_labels(vec![
                        Label::primary(file_id, span.clone())
                            .with_message(&e.message)
                    ]);
                }
                
                if let Some(suggestion) = &e.suggestion {
                    diag = diag.with_notes(vec![format!("help: {}", suggestion)]);
                }
                
                diag
            }
        };
        
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = codespan_reporting::term::Config::default();
        
        term::emit(&mut writer.lock(), &config, &files, &diagnostic)
            .expect("Failed to emit diagnostic");
    }
}

impl From<String> for FeraError {
    fn from(s: String) -> Self {
        FeraError::CodegenError(CodegenError {
            message: s,
            span: None,
            suggestion: None,
        })
    }
}

impl std::fmt::Display for FeraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeraError::LexError(e) => write!(f, "Lexer error: {}", e.message),
            FeraError::ParseError(e) => write!(f, "Parse error: {}", e.message),
            FeraError::TypeError(e) => write!(f, "Type error: {}", e.message),
            FeraError::CodegenError(e) => write!(f, "Codegen error: {}", e.message),
        }
    }
}

impl std::error::Error for FeraError {}

