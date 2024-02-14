use std::collections::HashMap;

use crate::ast::nodes::{AstType, GenericDecl, Node, AST, Expr, Stmt};
use crate::frontend::lexer::token::{Token, TokenType};
use crate::ast::attrs::{AstAttrs, AttrHandler, ExternalLinkage};
use crate::reports::{CompileError, Error, ErrorInfo, Reports};
use crate::ast::source::SourceLocation;
use crate::frontend::module::Module;

use super::lexer::Lexer;
use super::module::NamespacePath;

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    token: Token,
    reports: Reports,
    module: Module<Node>
}

macro_rules! report {
    ($self:ident, $error_type:expr) => { {
        $self.reports.add_error(CompileError::new($error_type, $self.token.get_location()));
        return Err(());
    }
    };
    ($self:ident, $error_type:expr, $info:expr) => {{
        $self.reports.add_error(CompileError::new($error_type, $self.token.get_location()).with_info($info));
        return Err(());
    }
    }
}

macro_rules! assert_token {
    ($self:ident, $expected:expr, $expectation:expr) => {{
        if *$self.token.get_type() != $expected {
            report!($self, Error::ExpectedItemAfter(Token::new($expected, SourceLocation::dummy()).value(), $expectation.to_string(), $self.token.value()));
        }
    }};
}

macro_rules! consume_token {
    ($self:ident, $expected:expr, $expectation:expr) => {{
        assert_token!($self, $expected, $expectation);
        $self.next();
    }};
}

impl Parser {
    pub fn new(path: NamespacePath, file_name: String, l: &Lexer) -> Parser {
        Parser {
            tokens: l.get_tokens().clone(),
            token_index: 0,
            token: l.get_tokens()[0].clone(),
            reports: Reports::new(),
            module: Module::new(path, Some(file_name))
        }
    }

    pub fn parse(&mut self) -> Result<Module<Node>, ()> {
        let nodes = self.parse_global(TokenType::EOF);
        self.module.set_top(AST::TopLevel(nodes?));
        Ok(self.module.clone())
    }

    pub fn assert_global_item_next(&mut self, after: String) -> Result<(), ()> {
        match self.token.get_type() {
            TokenType::Fn |
            TokenType::Struct |
            TokenType::Enum |
            TokenType::Class |
            TokenType::Const |
            TokenType::Public |
            TokenType::Private |
            TokenType::Static |
            TokenType::Inline |
            TokenType::External |
            TokenType::Abstract |
            TokenType::Final |
            TokenType::Interface => Ok(()),
            _ => report!(self, Error::ExpectedItem("global item".to_string(), after), ErrorInfo {
                help: Some("There are only a few items that can be declared at the global scope".to_string()),
                see: Some("https://snowball-lang.gitbook.io/docs/language-reference/global-scope".to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn parse_global(&mut self, terminator: TokenType) -> Result<Vec<Node>, ()> {
        let mut attrs = AttrHandler::new();
        let mut nodes = Vec::new();
        while *self.token.get_type() != terminator {
            match self.token.get_type() {
                TokenType::EOF => report!(self, Error::UnexpectedEOF),
                TokenType::Public => {
                    self.next();
                    attrs.add_attr(AstAttrs::Privacy(true));
                    self.assert_global_item_next("public".to_string())?;
                } 
                TokenType::Private => {
                    self.next();
                    attrs.add_attr(AstAttrs::Privacy(false));
                    self.assert_global_item_next("private".to_string())?;
                }
                TokenType::Static => {
                    self.next();
                    attrs.add_attr(AstAttrs::Static);
                    match self.token.get_type() {
                        TokenType::Fn |
                        TokenType::Const |
                        TokenType::External => {}
                        _ => report!(self, Error::ExpectedItem("function, constant, or external".to_string(), "static".to_string()), ErrorInfo {
                            help: Some("The 'static' keyword can only be used with functions, constants, or externs".to_string()),
                            note: Some("Static functions are used to specify that a function is a class method".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/static-functions".to_string()),
                            ..Default::default()
                        })
                    }
                }
                TokenType::Inline => {
                    self.next();
                    attrs.add_attr(AstAttrs::Inline);
                    match self.token.get_type() {
                        TokenType::Fn => {}
                        _ => report!(self, Error::ExpectedItem("function".to_string(), "inline".to_string()), ErrorInfo {
                            help: Some("The 'inline' keyword can only be used with functions".to_string()),
                            note: Some("Inline functions are used to specify that a function should be inlined by the compiler".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/inline-functions".to_string()),
                            ..Default::default()
                        }),
                    }
                }
                TokenType::External => {
                    self.next();
                    match self.token.get_type() {
                        TokenType::String(data) => {
                            match data.as_str() {
                                "C" => attrs.add_attr(AstAttrs::External(ExternalLinkage::C)),
                                "snowball" => attrs.add_attr(AstAttrs::External(ExternalLinkage::Snowball)),
                                "system" => attrs.add_attr(AstAttrs::External(ExternalLinkage::System)),
                                _ => report!(self, Error::InvalidExternalSpecifier(data.clone()), ErrorInfo {
                                    help: Some("The external specifier must be one of the following: 'C', 'snowball', 'system'".to_string()),
                                    info: Some("Not a valid external specifier!".to_string()),
                                    note: Some("External specifiers are used to specify the data that is being imported from an external source".to_string()),
                                    see: Some("https://snowball-lang.gitbook.io/docs/language-reference/external-specifier".to_string()),
                                    ..Default::default()
                                }),
                            }
                            self.next();
                        }
                        _ => report!(self, Error::ExpectedItem("external specifier".to_string(), "external".to_string()), ErrorInfo {
                            help: Some("The external specifier must be a string literal".to_string()),
                            note: Some("External specifiers are used to specify the data that is being imported from an external source".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/external-specifier".to_string()),
                            ..Default::default()
                        }),
                    }
                    match self.token.get_type() {
                        TokenType::Fn => {}
                        _ => report!(self, Error::ExpectedItem("function".to_string(), "external".to_string()), ErrorInfo {
                            help: Some("The 'external' keyword can only be used with functions".to_string()),
                            note: Some("External functions are used to specify that a function is being imported from an external source".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/external-functions".to_string()),
                            ..Default::default()
                        }),
                    }
                }
                TokenType::Abstract => {
                    self.next();
                    attrs.add_attr(AstAttrs::Abstract);
                    match self.token.get_type() {
                        TokenType::Class => {}
                        _ => report!(self, Error::ExpectedItem("class".to_string(), "abstract".to_string()), ErrorInfo {
                            help: Some("The 'abstract' keyword can only be used with classes".to_string()),
                            note: Some("Abstract classes are used to specify that a class cannot be instantiated".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/classes/abstract-classes".to_string()),
                            ..Default::default()
                        }),
                    }
                }
                TokenType::Final => {
                    self.next();
                    attrs.add_attr(AstAttrs::Final);
                    match self.token.get_type() {
                        TokenType::Class => {}
                        _ => report!(self, Error::ExpectedItem("class".to_string(), "final".to_string()), ErrorInfo {
                            help: Some("The 'final' keyword can only be used with classes".to_string()),
                            note: Some("Final classes are used to specify that a class cannot be inherited from".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/classes/final-classes".to_string()),
                            ..Default::default()
                        }),
                    }
                }
                TokenType::Fn => {
                    nodes.push(self.parse_function(attrs.clone())?);
                }
                _ => report!(self, Error::UnexpectedToken(self.token.value())),
            }
        }
        Ok(nodes)
    }

    pub fn parse_function(&mut self, attrs: AttrHandler) -> Result<Node, ()> {
        debug_assert!(*self.token.get_type() == TokenType::Fn);
        self.next();
        assert_token!(self, TokenType::Identifier("function name".to_string()), "function name");
        let name = self.token.value();
        self.next();
        let generics = self.parse_generic_args_if_present()?;
        consume_token!(self, TokenType::OpenParen, "function parameters");
        let mut params = HashMap::new();
        while *self.token.get_type() != TokenType::CloseParen {
            match self.token.get_type() {
                TokenType::Identifier(_) => {
                    let param = self.token.value();
                    if params.contains_key(&param) {
                        report!(self, Error::UnexpectedToken(param.clone()), ErrorInfo {
                            note: Some("Function parameters are used to specify the data that is being passed to a function".to_string()),
                            info: Some("Function parameters must be unique".to_string()),
                            ..Default::default()
                        });
                    }
                    self.next();
                    consume_token!(self, TokenType::Colon, "parameter separator");
                    let ty = self.parse_type()?;
                    params.insert(param, ty);
                }
                _ => report!(self, Error::ExpectedItem("parameter".to_string(), "function parameter".to_string()), ErrorInfo {
                    help: Some("Function parameters must be identifiers".to_string()),
                    note: Some("Function parameters are used to specify the data that is being passed to a function".to_string()),
                    info: Some("Expected an identifier for the function parameter".to_string()),
                    ..Default::default()
                }),
            }
        }
        self.next();
        let ret_ty = match self.token.get_type() {
            TokenType::OpenBrace |
            TokenType::Semicolon => AstType::new(Node::new(self.create_expr_ast(Expr::Ident("void".to_string(), None)))),
            _ => self.parse_type()?,
        };
        let body = Some(self.parse_block()?);
        Ok(Node::new(self.create_stmt_ast(Stmt::FuncDef(name, params, ret_ty, body, generics))).with_attrs(attrs).clone())
    }

    pub fn parse_block(&mut self) -> Result<Node, ()> {
        consume_token!(self, TokenType::OpenBrace, "block");
        let mut nodes = Vec::new();
        while *self.token.get_type() != TokenType::CloseBrace {
            nodes.push(self.parse_statement()?);
        }
        self.next();
        Ok(Node::new(self.create_stmt_ast(Stmt::Block(nodes))))
    }

    pub fn parse_statement(&mut self) -> Result<Node, ()> {
        match self.token.get_type() {
            TokenType::Return => {
                self.next();
                let expr = match self.token.get_type() {
                    TokenType::Semicolon => None,
                    _ => Some(self.parse_expression()?),
                };
                consume_token!(self, TokenType::Semicolon, "return statement");
                Ok(Node::new(self.create_stmt_ast(Stmt::Return(expr))))
            }
            TokenType::Break => {
                self.next();
                consume_token!(self, TokenType::Semicolon, "break statement");
                Ok(Node::new(self.create_stmt_ast(Stmt::Break)))
            }
            TokenType::Continue => {
                self.next();
                consume_token!(self, TokenType::Semicolon, "continue statement");
                Ok(Node::new(self.create_stmt_ast(Stmt::Continue)))
            }
            TokenType::If => {
                self.next();
                consume_token!(self, TokenType::OpenParen, "if statement");
                let cond = self.parse_expression()?;
                consume_token!(self, TokenType::CloseParen, "if statement");
                let then = self.parse_statement()?;
                let mut els = Vec::new();
                while *self.token.get_type() == TokenType::Else {
                    self.next();
                    if *self.token.get_type() == TokenType::If {
                        self.next();
                        consume_token!(self, TokenType::OpenParen, "else if statement");
                        let cond = self.parse_expression()?;
                        consume_token!(self, TokenType::CloseParen, "else if statement");
                        let then = self.parse_statement()?;
                        els.push(Node::new(self.create_stmt_ast(Stmt::If(cond, then, Vec::new()))));
                    } else {
                        let stmt = self.parse_statement()?;
                        els.push(stmt);
                    }
                }
                Ok(Node::new(self.create_stmt_ast(Stmt::If(cond, then, els))))
            }
            TokenType::While => {
                self.next();
                consume_token!(self, TokenType::OpenParen, "while statement");
                let cond = self.parse_expression()?;
                consume_token!(self, TokenType::CloseParen, "while statement");
                let body = self.parse_statement()?;
                Ok(Node::new(self.create_stmt_ast(Stmt::While(cond, vec![body], false))))
            }
            TokenType::Do => {
                self.next();
                let body = self.parse_statement()?;
                consume_token!(self, TokenType::While, "do while statement");
                consume_token!(self, TokenType::OpenParen, "do while statement");
                let cond = self.parse_expression()?;
                consume_token!(self, TokenType::CloseParen, "do while statement");
                consume_token!(self, TokenType::Semicolon, "do while statement");
                Ok(Node::new(self.create_stmt_ast(Stmt::While(cond, vec![body], true))))
            }
            TokenType::OpenBrace => self.parse_block(),
            _ => {
                let expr = self.parse_expression()?;
                consume_token!(self, TokenType::Semicolon, "expression statement");
                Ok(expr)
            }
        }
    }

    pub fn parse_expression(&mut self) -> Result<Node, ()> {
        match self.token.get_type() {
            TokenType::Identifier(_) => {
                let name = self.token.value();
                self.next();
                match self.token.get_type() {
                    _ => Ok(Node::new(self.create_expr_ast(Expr::Ident(name, None)))),
                }
            }
            TokenType::Integer(_) => {
                let value = self.token.value();
                self.next();
                // parse the number to an i64
                let value = value.parse::<i64>().unwrap();
                Ok(Node::new(self.create_expr_ast(Expr::Int(value))))
            }
            TokenType::Float(_) => {
                let value = self.token.value();
                self.next();
                // parse the number to a f64
                let value = value.parse::<f64>().unwrap();
                Ok(Node::new(self.create_expr_ast(Expr::Float(value))))
            }
            TokenType::String(_) => {
                let value = self.token.value();
                self.next();
                Ok(Node::new(self.create_expr_ast(Expr::String(value))))
            }
            TokenType::OpenParen => {
                self.next();
                let expr = self.parse_expression()?;
                consume_token!(self, TokenType::CloseParen, "parenthesized expression");
                Ok(expr)
            }
            _ => report!(self, Error::UnexpectedToken(self.token.value())),
        }
    }

    pub fn parse_type(&mut self) -> Result<AstType, ()> {
        match self.token.get_type() {
            TokenType::Identifier(_) => {
                let name = self.token.value();
                self.next();
                let mut generics = None;
                while *self.token.get_type() == TokenType::LessThan {
                    generics = Some(Vec::new());
                    self.next();
                    while *self.token.get_type() != TokenType::GreaterThan {
                        generics.as_mut().unwrap().push(self.parse_type()?);
                        if *self.token.get_type() == TokenType::Comma {
                            self.next();
                        }
                    }
                    self.next();
                }
                Ok(AstType::new(Node::new(self.create_expr_ast(Expr::Ident(name, generics)))))
            }
            _ => report!(self, Error::ExpectedItem("type".to_string(), "type".to_string()), ErrorInfo {
                help: Some("Types can be identifiers, tuples, or arrays".to_string()),
                note: Some("Types are used to specify the data that is being passed to a function or stored in a variable".to_string()),
                info: Some("Expected an identifier, tuple, or array for the type".to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn parse_generic_args_if_present(&mut self) -> Result<Option<Vec<GenericDecl>>, ()> {
        if let TokenType::LessThan = self.token.get_type() {
            self.next();
            let mut generics = None;
            while *self.token.get_type() != TokenType::GreaterThan {
                generics = Some(Vec::new());
                match self.token.get_type() {
                    TokenType::Identifier(_) => {
                        let name = self.token.value();
                        self.next();
                        let mut impls = Vec::new();
                        let mut default = None;
                        if let TokenType::Colon = self.token.get_type() {
                            self.next();
                            impls.push(self.parse_type()?);
                            while *self.token.get_type() == TokenType::Pipe {
                                self.next();
                                impls.push(self.parse_type()?);
                            }
                        }
                        if let TokenType::Equal = self.token.get_type() {
                            self.next();
                            default = Some(self.parse_type()?);
                        }
                        generics.as_mut().unwrap().push(GenericDecl::new(name, impls, default));
                    }
                    _ => report!(self, Error::ExpectedItem("generic argument".to_string(), "generic argument".to_string()), ErrorInfo {
                        help: Some("Generic arguments must be identifiers".to_string()),
                        note: Some("Generic arguments are used to specify the data that is being passed to a generic type".to_string()),
                        info: Some("Expected an identifier for the generic argument".to_string()),
                        ..Default::default()
                    }),
                }
            }
            self.next();
            return Ok(generics.clone());
        } 
        Ok(None)
    }

    pub fn create_expr_ast(&mut self, ast: Expr) -> AST {
        AST::Expr(ast)
    }

    pub fn create_stmt_ast(&mut self, ast: Stmt) -> AST {
        AST::Stmt(ast)
    }
    
    pub fn next(&mut self) {
        self.token_index += 1;
        self.token = self.tokens[self.token_index].clone();
    }

    pub fn get_reports(&self) -> &Reports {
        &self.reports
    }
}

