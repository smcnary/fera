use crate::ast::*;
use crate::lexer::token::{Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    current: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens: tokens.into_iter().peekable(),
            current: None,
        };
        parser.advance();
        parser
    }
    
    fn advance(&mut self) -> Option<Token> {
        self.current = self.tokens.next();
        self.current.clone()
    }
    
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    
    fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        if let Some(token) = &self.current {
            if token.kind == kind {
                let token = token.clone();
                self.advance();
                return Ok(token);
            }
        }
        Err(format!("Expected {:?}, found {:?}", kind, self.current))
    }
    
    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut items = Vec::new();
        
        while let Some(token) = &self.current {
            if token.kind == TokenKind::Eof {
                break;
            }
            items.push(self.parse_item()?);
        }
        
        Ok(Program { items })
    }
    
    fn parse_item(&mut self) -> Result<Item, String> {
        let token = self.current.as_ref().ok_or("Unexpected EOF")?;
        
        match &token.kind {
            TokenKind::Export | TokenKind::Internal => {
                self.parse_function_or_global()
            }
            TokenKind::Struct => Ok(Item::Struct(self.parse_struct()?)),
            TokenKind::Union => Ok(Item::Union(self.parse_union()?)),
            TokenKind::Enum => Ok(Item::Enum(self.parse_enum()?)),
            TokenKind::Typedef => Ok(Item::TypeDef(self.parse_typedef()?)),
            _ => self.parse_function_or_global(),
        }
    }
    
    fn parse_function_or_global(&mut self) -> Result<Item, String> {
        let linkage = if let Some(token) = &self.current {
            match token.kind {
                TokenKind::Export => {
                    self.advance();
                    Linkage::Export
                }
                TokenKind::Internal => {
                    self.advance();
                    Linkage::Internal
                }
                _ => Linkage::Export, // default
            }
        } else {
            Linkage::Export
        };
        
        let return_type = self.parse_type()?;
        let name_token = self.expect(TokenKind::Identifier)?;
        let name = name_token.text.clone();
        
        if let Some(token) = &self.current {
            if token.kind == TokenKind::LeftParen {
                // Function
                self.advance(); // (
                let params = self.parse_params()?;
                self.expect(TokenKind::RightParen)?;
                
                let body = if let Some(token) = &self.current {
                    if token.kind == TokenKind::LeftBrace {
                        Some(self.parse_block()?)
                    } else {
                        self.expect(TokenKind::Semicolon)?;
                        None
                    }
                } else {
                    None
                };
                
                Ok(Item::Function(Function {
                    linkage,
                    return_type,
                    name,
                    params,
                    body,
                    attributes: Vec::new(),
                    span: 0..0, // TODO: proper span tracking
                }))
            } else {
                // Global variable
                let init = if let Some(token) = &self.current {
                    if token.kind == TokenKind::Equals {
                        self.advance();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                self.expect(TokenKind::Semicolon)?;
                
                Ok(Item::GlobalVar(GlobalVar {
                    linkage,
                    ty: return_type,
                    name,
                    init,
                    span: 0..0,
                }))
            }
        } else {
            Err("Unexpected EOF".to_string())
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, String> {
        let token = self.current.as_ref().ok_or("Expected type")?;
        
        let base_type = match &token.kind {
            TokenKind::Void => { self.advance(); Type::Void }
            TokenKind::Bool => { self.advance(); Type::Bool }
            TokenKind::Char => { self.advance(); Type::Char }
            TokenKind::Int => { self.advance(); Type::I32 }
            TokenKind::I8 => { self.advance(); Type::I8 }
            TokenKind::I16 => { self.advance(); Type::I16 }
            TokenKind::I32 => { self.advance(); Type::I32 }
            TokenKind::I64 => { self.advance(); Type::I64 }
            TokenKind::ISize => { self.advance(); Type::ISize }
            TokenKind::U8 => { self.advance(); Type::U8 }
            TokenKind::U16 => { self.advance(); Type::U16 }
            TokenKind::U32 => { self.advance(); Type::U32 }
            TokenKind::U64 => { self.advance(); Type::U64 }
            TokenKind::USize => { self.advance(); Type::USize }
            TokenKind::F32 => { self.advance(); Type::F32 }
            TokenKind::F64 => { self.advance(); Type::F64 }
            TokenKind::Struct => {
                self.advance();
                let name = self.expect(TokenKind::Identifier)?;
                Type::Struct(name.text)
            }
            TokenKind::Enum => {
                self.advance();
                let name = self.expect(TokenKind::Identifier)?;
                Type::Enum(name.text)
            }
            TokenKind::Identifier => {
                let name = token.text.clone();
                self.advance();
                Type::Named(name)
            }
            _ => return Err(format!("Expected type, found {:?}", token.kind)),
        };
        
        // Handle pointer, array, etc.
        self.parse_type_suffix(base_type)
    }
    
    fn parse_type_suffix(&mut self, base: Type) -> Result<Type, String> {
        if let Some(token) = &self.current {
            match token.kind {
                TokenKind::Star => {
                    self.advance();
                    let qualifiers = self.parse_type_qualifiers();
                    Ok(Type::Pointer(Box::new(base), qualifiers))
                }
                TokenKind::LeftBracket => {
                    self.advance();
                    let size = if let Some(token) = &self.current {
                        if token.kind == TokenKind::RightBracket {
                            None
                        } else {
                            // Parse size expression (simplified for now)
                            self.advance();
                            Some(0) // TODO: parse actual size
                        }
                    } else {
                        None
                    };
                    self.expect(TokenKind::RightBracket)?;
                    Ok(Type::Array(Box::new(base), size))
                }
                _ => Ok(base),
            }
        } else {
            Ok(base)
        }
    }
    
    fn parse_type_qualifiers(&mut self) -> Vec<TypeQualifier> {
        let mut qualifiers = Vec::new();
        
        while let Some(token) = &self.current {
            match token.kind {
                TokenKind::Const => {
                    self.advance();
                    qualifiers.push(TypeQualifier::Const);
                }
                TokenKind::Volatile => {
                    self.advance();
                    qualifiers.push(TypeQualifier::Volatile);
                }
                TokenKind::Restrict => {
                    self.advance();
                    qualifiers.push(TypeQualifier::Restrict);
                }
                _ => break,
            }
        }
        
        qualifiers
    }
    
    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightParen {
                break;
            }
            
            let ty = self.parse_type()?;
            let name = self.expect(TokenKind::Identifier)?;
            
            params.push(Param {
                ty,
                name: name.text,
                span: 0..0,
            });
            
            if let Some(token) = &self.current {
                if token.kind == TokenKind::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        Ok(params)
    }
    
    fn parse_struct(&mut self) -> Result<Struct, String> {
        self.expect(TokenKind::Struct)?;
        let name = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::LeftBrace)?;
        
        let mut fields = Vec::new();
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            let ty = self.parse_type()?;
            let name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Semicolon)?;
            
            fields.push(Field {
                ty,
                name: name.text,
                span: 0..0,
            });
        }
        
        self.expect(TokenKind::RightBrace)?;
        self.expect(TokenKind::Semicolon)?;
        
        Ok(Struct {
            name: name.text,
            fields,
            attributes: Vec::new(),
            span: 0..0,
        })
    }
    
    fn parse_union(&mut self) -> Result<Union, String> {
        self.expect(TokenKind::Union)?;
        let name = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::LeftBrace)?;
        
        let mut fields = Vec::new();
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            let ty = self.parse_type()?;
            let name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Semicolon)?;
            
            fields.push(Field {
                ty,
                name: name.text,
                span: 0..0,
            });
        }
        
        self.expect(TokenKind::RightBrace)?;
        self.expect(TokenKind::Semicolon)?;
        
        Ok(Union {
            name: name.text,
            fields,
            span: 0..0,
        })
    }
    
    fn parse_enum(&mut self) -> Result<Enum, String> {
        self.expect(TokenKind::Enum)?;
        let name = self.expect(TokenKind::Identifier)?;
        
        let backing_type = if let Some(token) = &self.current {
            if token.kind == TokenKind::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            }
        } else {
            None
        };
        
        self.expect(TokenKind::LeftBrace)?;
        
        let mut variants = Vec::new();
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            let variant_name = self.expect(TokenKind::Identifier)?;
            let value = if let Some(token) = &self.current {
                if token.kind == TokenKind::Equals {
                    self.advance();
                    Some(self.parse_expr()?)
                } else {
                    None
                }
            } else {
                None
            };
            
            variants.push(EnumVariant {
                name: variant_name.text,
                value,
                span: 0..0,
            });
            
            if let Some(token) = &self.current {
                if token.kind == TokenKind::Comma {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenKind::RightBrace)?;
        self.expect(TokenKind::Semicolon)?;
        
        Ok(Enum {
            name: name.text,
            backing_type,
            variants,
            span: 0..0,
        })
    }
    
    fn parse_typedef(&mut self) -> Result<TypeDef, String> {
        self.expect(TokenKind::Typedef)?;
        let ty = self.parse_type()?;
        let name = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::Semicolon)?;
        
        Ok(TypeDef {
            name: name.text,
            ty,
            span: 0..0,
        })
    }
    
    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(TokenKind::LeftBrace)?;
        
        let mut stmts = Vec::new();
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            stmts.push(self.parse_stmt()?);
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Block { stmts, span: 0..0 })
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let token = self.current.as_ref().ok_or("Expected statement")?;
        
        match &token.kind {
            TokenKind::Return => {
                self.advance();
                let value = if let Some(token) = &self.current {
                    if token.kind == TokenKind::Semicolon {
                        None
                    } else {
                        Some(self.parse_expr()?)
                    }
                } else {
                    None
                };
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Return(value, 0..0))
            }
            TokenKind::If => Ok(Stmt::If(self.parse_if()?)),
            TokenKind::While => Ok(Stmt::While(self.parse_while()?)),
            TokenKind::For => Ok(Stmt::For(self.parse_for()?)),
            TokenKind::Break => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Break(0..0))
            }
            TokenKind::Continue => {
                self.advance();
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Continue(0..0))
            }
            TokenKind::LeftBrace => Ok(Stmt::Block(self.parse_block()?)),
            _ => {
                // Try to parse as variable declaration or expression
                if self.is_type_start() {
                    self.parse_let_stmt()
                } else {
                    let expr = self.parse_expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }
    
    fn is_type_start(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind, 
                TokenKind::Void | TokenKind::Bool | TokenKind::Char | TokenKind::Int |
                TokenKind::I8 | TokenKind::I16 | TokenKind::I32 | TokenKind::I64 | TokenKind::ISize |
                TokenKind::U8 | TokenKind::U16 | TokenKind::U32 | TokenKind::U64 | TokenKind::USize |
                TokenKind::F32 | TokenKind::F64 | TokenKind::Struct | TokenKind::Enum
            )
        } else {
            false
        }
    }
    
    fn parse_let_stmt(&mut self) -> Result<Stmt, String> {
        let ty = self.parse_type()?;
        let name = self.expect(TokenKind::Identifier)?;
        
        let init = if let Some(token) = &self.current {
            if token.kind == TokenKind::Equals {
                self.advance();
                Some(self.parse_expr()?)
            } else {
                None
            }
        } else {
            None
        };
        
        self.expect(TokenKind::Semicolon)?;
        
        Ok(Stmt::Let(Let {
            ty,
            name: name.text,
            init,
            span: 0..0,
        }))
    }
    
    fn parse_if(&mut self) -> Result<If, String> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::RightParen)?;
        
        let then_branch = Box::new(self.parse_stmt()?);
        
        let else_branch = if let Some(token) = &self.current {
            if token.kind == TokenKind::Else {
                self.advance();
                Some(Box::new(self.parse_stmt()?))
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(If {
            condition,
            then_branch,
            else_branch,
            span: 0..0,
        })
    }
    
    fn parse_while(&mut self) -> Result<While, String> {
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::LeftParen)?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::RightParen)?;
        let body = Box::new(self.parse_stmt()?);
        
        Ok(While { condition, body, span: 0..0 })
    }
    
    fn parse_for(&mut self) -> Result<For, String> {
        self.expect(TokenKind::For)?;
        self.expect(TokenKind::LeftParen)?;
        
        let init = if let Some(token) = &self.current {
            if token.kind == TokenKind::Semicolon {
                None
            } else {
                Some(Box::new(self.parse_stmt()?))
            }
        } else {
            None
        };
        
        let condition = if let Some(token) = &self.current {
            if token.kind == TokenKind::Semicolon {
                None
            } else {
                Some(self.parse_expr()?)
            }
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        
        let increment = if let Some(token) = &self.current {
            if token.kind == TokenKind::RightParen {
                None
            } else {
                Some(self.parse_expr()?)
            }
        } else {
            None
        };
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_stmt()?);
        
        Ok(For {
            init,
            condition,
            increment,
            body,
            span: 0..0,
        })
    }
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_ternary()?;
        
        if let Some(token) = &self.current {
            if token.kind == TokenKind::Equals {
                self.advance();
                let rhs = self.parse_assignment()?;
                return Ok(Expr::Assign(Box::new(expr), Box::new(rhs), 0..0));
            }
        }
        
        Ok(expr)
    }
    
    fn parse_ternary(&mut self) -> Result<Expr, String> {
        let expr = self.parse_logical_or()?;
        
        if let Some(token) = &self.current {
            if token.kind == TokenKind::Question {
                self.advance();
                let then_expr = self.parse_expr()?;
                self.expect(TokenKind::Colon)?;
                let else_expr = self.parse_ternary()?;
                return Ok(Expr::Ternary(
                    Box::new(expr),
                    Box::new(then_expr),
                    Box::new(else_expr),
                    0..0,
                ));
            }
        }
        
        Ok(expr)
    }
    
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;
        
        while let Some(token) = &self.current {
            if token.kind == TokenKind::LogicalOr {
                self.advance();
                let right = self.parse_logical_and()?;
                left = Expr::Binary(BinaryOp::LogicalOr, Box::new(left), Box::new(right), 0..0);
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        
        while let Some(token) = &self.current {
            if token.kind == TokenKind::LogicalAnd {
                self.advance();
                let right = self.parse_equality()?;
                left = Expr::Binary(BinaryOp::LogicalAnd, Box::new(left), Box::new(right), 0..0);
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        
        while let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::EqualEqual => BinaryOp::Equal,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), 0..0);
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        
        while let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::Less => BinaryOp::Less,
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), 0..0);
        }
        
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        
        while let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), 0..0);
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        
        while let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), 0..0);
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some(token) = &self.current {
            let op = match token.kind {
                TokenKind::Minus => Some(UnaryOp::Neg),
                TokenKind::Bang => Some(UnaryOp::Not),
                TokenKind::Tilde => Some(UnaryOp::BitNot),
                TokenKind::Star => Some(UnaryOp::Deref),
                TokenKind::Ampersand => Some(UnaryOp::AddrOf),
                _ => None,
            };
            
            if let Some(op) = op {
                self.advance();
                let expr = self.parse_unary()?;
                return Ok(Expr::Unary(op, Box::new(expr), 0..0));
            }
        }
        
        self.parse_postfix()
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if let Some(token) = &self.current {
                match token.kind {
                    TokenKind::LeftParen => {
                        self.advance();
                        let args = self.parse_args()?;
                        self.expect(TokenKind::RightParen)?;
                        expr = Expr::Call(Box::new(expr), args, 0..0);
                    }
                    TokenKind::LeftBracket => {
                        self.advance();
                        let index = self.parse_expr()?;
                        self.expect(TokenKind::RightBracket)?;
                        expr = Expr::Index(Box::new(expr), Box::new(index), 0..0);
                    }
                    TokenKind::Dot => {
                        self.advance();
                        let field = self.expect(TokenKind::Identifier)?;
                        expr = Expr::Field(Box::new(expr), field.text, 0..0);
                    }
                    TokenKind::Arrow => {
                        self.advance();
                        let field = self.expect(TokenKind::Identifier)?;
                        // ptr->field is syntactic sugar for (*ptr).field
                        let deref = Expr::Unary(UnaryOp::Deref, Box::new(expr), 0..0);
                        expr = Expr::Field(Box::new(deref), field.text, 0..0);
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        
        while let Some(token) = &self.current {
            if token.kind == TokenKind::RightParen {
                break;
            }
            
            args.push(self.parse_expr()?);
            
            if let Some(token) = &self.current {
                if token.kind == TokenKind::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        Ok(args)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.current.as_ref().ok_or("Unexpected EOF")?;
        
        match &token.kind {
            TokenKind::DecimalInteger | TokenKind::HexInteger | 
            TokenKind::OctalInteger | TokenKind::BinaryInteger => {
                let value = self.parse_integer(&token.text)?;
                self.advance();
                Ok(Expr::IntLiteral(value, 0..0))
            }
            TokenKind::FloatLiteral => {
                let value = token.text.trim_end_matches('f')
                    .parse::<f64>()
                    .map_err(|e| format!("Invalid float: {}", e))?;
                self.advance();
                Ok(Expr::FloatLiteral(value, 0..0))
            }
            TokenKind::StringLiteral => {
                let value = token.text[1..token.text.len()-1].to_string();
                self.advance();
                Ok(Expr::StringLiteral(value, 0..0))
            }
            TokenKind::CharLiteral => {
                let value = token.text.chars().nth(1).ok_or("Invalid char")?;
                self.advance();
                Ok(Expr::CharLiteral(value, 0..0))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::BoolLiteral(true, 0..0))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::BoolLiteral(false, 0..0))
            }
            TokenKind::Identifier => {
                let name = token.text.clone();
                self.advance();
                Ok(Expr::Identifier(name, 0..0))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", token.kind)),
        }
    }
    
    fn parse_integer(&self, text: &str) -> Result<i64, String> {
        let text = text.trim_end_matches(|c| c == 'u' || c == 'U' || c == 'l' || c == 'L');
        
        if text.starts_with("0x") || text.starts_with("0X") {
            i64::from_str_radix(&text[2..], 16)
                .map_err(|e| format!("Invalid hex integer: {}", e))
        } else if text.starts_with("0o") || text.starts_with("0O") {
            i64::from_str_radix(&text[2..], 8)
                .map_err(|e| format!("Invalid octal integer: {}", e))
        } else if text.starts_with("0b") || text.starts_with("0B") {
            i64::from_str_radix(&text[2..], 2)
                .map_err(|e| format!("Invalid binary integer: {}", e))
        } else {
            text.parse::<i64>()
                .map_err(|e| format!("Invalid decimal integer: {}", e))
        }
    }
}

