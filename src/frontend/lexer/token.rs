#[derive(Clone, Debug)]
pub enum TokenType {
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Char(u8),
    Continue,
    Break,
    Return,
    If,
    Else,
    While,
    For,
    Let,
    Mut,
    Struct,
    Enum,
    Class,
    Do,
    Interface,
    Public,
    Private,
    Import,
    Const,
    Static,
    Inline,
    Abstract,
    Override,
    Final,
    External,
    True,
    False,
    Fn,
    New,
    Super,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    At,
    Arrow,
    DoubleColon,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmpersandEqual,
    PipeEqual,
    Equal,
    DoubleEqual,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    DoublePlus,
    DoubleMinus,
    DoubleAmpersand,
    DoublePipe,
    DoubleLessThan,
    DoubleGreaterThan,
    DoubleLessThanEqual,
    DoubleGreaterThanEqual,
    Question,
    Exclamation,
    Tilde,
    Hash,
    EOF,
}

impl PartialEq for TokenType {
  fn eq(&self, other: &Self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(other)
  }
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    location: crate::ast::source::SourceLocation,
}

impl Token {
    pub fn new(token_type: TokenType, location: crate::ast::source::SourceLocation) -> Token {
        Token {
            token_type,
            location,
        }
    }

    pub fn get_location(&self) -> crate::ast::source::SourceLocation {
        self.location.clone()
    }

    pub fn set_location(&mut self, location: crate::ast::source::SourceLocation) {
        self.location = location;
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn value(&self) -> String {
        match &self.token_type {
            TokenType::Identifier(value) => value.clone(),
            TokenType::Override => String::from("override"),
            TokenType::Integer(value) => value.clone(),
            TokenType::Float(value) => value.clone(),
            TokenType::String(value) => format!("\"{}\"", value),
            TokenType::Char(value) => format!("'{}'", (*value) as char),
            TokenType::Continue => String::from("continue"),
            TokenType::Break => String::from("break"),
            TokenType::Return => String::from("return"),
            TokenType::If => String::from("if"),
            TokenType::Else => String::from("else"),
            TokenType::While => String::from("while"),
            TokenType::For => String::from("for"),
            TokenType::Let => String::from("let"),
            TokenType::Mut => String::from("mut"),
            TokenType::Struct => String::from("struct"),
            TokenType::Enum => String::from("enum"),
            TokenType::Class => String::from("class"),
            TokenType::Interface => String::from("interface"),
            TokenType::Public => String::from("public"),
            TokenType::Private => String::from("private"),
            TokenType::Import => String::from("import"),
            TokenType::Const => String::from("const"),
            TokenType::Static => String::from("static"),
            TokenType::Inline => String::from("inline"),
            TokenType::True => String::from("true"),
            TokenType::External => String::from("external"),
            TokenType::Abstract => String::from("abstract"),
            TokenType::Final => String::from("final"),
            TokenType::False => String::from("false"),
            TokenType::Fn => String::from("func"),
            TokenType::New => String::from("new"),
            TokenType::Super => String::from("super"),
            TokenType::OpenParen => String::from("("),
            TokenType::CloseParen => String::from(")"),
            TokenType::OpenBrace => String::from("{"),
            TokenType::CloseBrace => String::from("}"),
            TokenType::OpenBracket => String::from("["),
            TokenType::CloseBracket => String::from("]"),
            TokenType::Semicolon => String::from(";"),
            TokenType::Colon => String::from(":"),
            TokenType::Comma => String::from(","),
            TokenType::Dot => String::from("."),
            TokenType::Plus => String::from("+"),
            TokenType::Minus => String::from("-"),
            TokenType::Star => String::from("*"),
            TokenType::Do => String::from("do"),
            TokenType::Slash => String::from("/"),
            TokenType::Percent => String::from("%"),
            TokenType::Ampersand => String::from("&"),
            TokenType::Pipe => String::from("|"),
            TokenType::At => String::from("@"),
            TokenType::Arrow => String::from("=>"),
            TokenType::DoubleColon => String::from("::"),
            TokenType::PlusEqual => String::from("+="),
            TokenType::MinusEqual => String::from("-="),
            TokenType::StarEqual => String::from("*="),
            TokenType::SlashEqual => String::from("/="),
            TokenType::PercentEqual => String::from("%="),
            TokenType::AmpersandEqual => String::from("&="),
            TokenType::PipeEqual => String::from("|="),
            TokenType::Equal => String::from("="),
            TokenType::DoubleEqual => String::from("=="),
            TokenType::NotEqual => String::from("!="),
            TokenType::LessThan => String::from("<"),
            TokenType::LessThanEqual => String::from("<="),
            TokenType::GreaterThan => String::from(">"),
            TokenType::GreaterThanEqual => String::from(">="),
            TokenType::DoublePlus => String::from("++"),
            TokenType::DoubleMinus => String::from("--"),
            TokenType::DoubleAmpersand => String::from("&&"),
            TokenType::DoublePipe => String::from("||"),
            TokenType::DoubleLessThan => String::from("<<"),
            TokenType::DoubleGreaterThan => String::from(">>"),
            TokenType::DoubleLessThanEqual => String::from("<<="),
            TokenType::DoubleGreaterThanEqual => String::from(">>="),
            TokenType::Question => String::from("?"),
            TokenType::Exclamation => String::from("!"),
            TokenType::Tilde => String::from("~"),
            TokenType::Hash => String::from("#"),
            TokenType::EOF => String::from("<EOF>"),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Token({})", self.value())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
