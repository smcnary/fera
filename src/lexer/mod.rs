pub mod token;
pub mod preprocessor;

use logos::Logos;
use std::fmt;
use crate::lexer::token::{Token, TokenKind};

pub struct Lexer<'source> {
    source: &'source str,
    lexer: logos::Lexer<'source, TokenKind>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            lexer: TokenKind::lexer(source),
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(kind) = self.lexer.next() {
            let span = self.lexer.span();
            let text = &self.source[span.clone()];
            let kind = kind.unwrap_or(TokenKind::Error);
            
            // Skip comments and preprocessor directives
            match kind {
                TokenKind::LineComment | TokenKind::BlockComment | TokenKind::PreprocessorDirective => {
                    continue;
                }
                _ => {}
            }
            
            tokens.push(Token {
                kind,
                text: text.to_string(),
                span: span.start..span.end,
            });
        }
        
        // Add EOF token
        let eof_pos = self.source.len();
        tokens.push(Token {
            kind: TokenKind::Eof,
            text: String::new(),
            span: eof_pos..eof_pos,
        });
        
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let source = "int main() { return 0; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        assert!(tokens.len() > 0);
        assert!(matches!(tokens[0].kind, TokenKind::Int));
    }
}

