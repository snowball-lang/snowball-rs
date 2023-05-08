use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokType {
    Ident,

    // Literal values
    ValNum(i32),
    ValFloat(f32), // TODO: can contain "E" and other shit
    ValBool(bool),
    ValString(String),
    ValChar(char),

    SymbolAt,        // Symbol: @
    SymbolDot,       // Symbol: .
    SymbolHash,      // Symbol: #
    SymbolComma,     // Symbol: ,
    SymbolColon,     // Symbol: :
    SymbolColcol,    // Symbol: ::
    SymbolDollar,    // Symbol: $
    SymbolQuestion,  // Symbol: ?
    SymbolSemiColon, // Symbol: ;

    // |- BRACKETS -|
    BracketLCurly,   // Symbol: {
    BracketRCurly,   // Symbol: }
    BracketLParen,   // Symbol: (
    BracketRParen,   // Symbol: )
    BracketRSquared, // Symbol: [
    BracketLSquared, // Symbol: ]
    OpMul,           // Symbol: *
    OpMod,           // Symbol: %
    OpDiv,           // Symbol: /
    OpPlus,          // Symbol: +
    OpMinus,         // Symbol: -

    // Double characters
    OpMulEq,   // Symbol: *=
    OpDivEq,   // Symbol: /=
    OpModEq,   // Symbol: %=
    OpPlusEq,  // Symbol: +=
    OpMinusEq, // Symbol: -=

    // Single character tokens
    OpGt, // Symbol: >
    OpLt, // Symbol: <

    // Double character tokens
    OpArrow, // Symbol: =>
    OpEqEq,  // Symbol: ==
    OpGtEq,  // Symbol: >=
    OpLtEq,  // Symbol: <=
    OpNotEq, // Symbol: !=

    // Single character tokens
    OpEq,  // Symbol: =
    OpNot, // Symbol: !

    // Double character tokens
    OpAnd, // Symbol: &&
    OpOr,  // Symbol: ||

    // Bitwise operations
    OpBitNot,   // Symbol: ~
    OpBitOr,    // Symbol: |
    OpBitAnd,   // Symbol: &
    OpBitXor,   // Symbol: ^
    OpBitOrEq,  // Symbol: |=
    OpBitRsh,   // Symbol: >>
    OpBitLsh,   // Symbol: <<
    OpBitAndEq, // Symbol: &=
    OpBitXorEq, // Symbol: ^=
    OpBitRshEq, // Symbol: >>=
    OpBitLshEq, // Symbol: <<=

    KWordIf,       // Symbol: if
    KWordVar,      // Symbol: var
    KWordNew,      // Symbol: new
    KWordFor,      // Symbol: for
    KWordEnum,     // Symbol: enum
    KWordFunc,     // Symbol: fn
    KWordOperator, // Symbol: operator
    KWordElse,     // Symbol: else
    KWordCase,     // Symbol: case
    KWordBreak,    // Symbol: break
    KWordConst,    // Symbol: const
    KWordSuper,    // Symbol: super
    KWordWhile,    // Symbol: while
    KWordExtern,   // Synbol: extern
    KWordVirtual,  // Synbol: virtual
    KWordClass,    // Symbol: class
    KWordAs,       // Symbol: as
    KWordImport,   // Symbol: use
    KWordPub,      // Symbol: pub
    KWordSwitch,   // Symbol: switch
    KWordStatic,   // Symbol: static
    KWordReturn,   // Symbol: return
    KWordPrivate,  // Symbol: priv
    KWordDefault,  // Symbol: default
    KWordTry,      // Symbol: try
    KWordCatch,    // Symbol: catch
    KWordContinue, // Symbol: continue,

    EOF, // Just an EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub pos: usize,
    pub tok: TokType,
    pub value: String,
}

pub fn tokenise(code: String, filename: String) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    lexer.tokenise()
}

struct Lexer {
    text: Vec<char>,
    pos: usize,
    tokens: Vec<Token>,

    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            text: code.chars().collect(),
            line: 0,
            pos: 0,
            col: 0,
            tokens: Vec::new(),
        }
    }

    fn consume(&mut self, ty: TokType, value: &str) {
        self.pos += value.len();

        let token = Token {
            line: self.line,
            pos: self.col,
            tok: ty,
            value: value.to_string(),
        };

        self.tokens.push(token);
    }

    fn get_char(&self, offset: Option<usize>) -> char {
        // use Option<T> not that shit
        let new_pos = self.pos + offset.unwrap_or(0);
        println!("{}", self.text.len());
        if self.text.len() - 1 <= new_pos.try_into().unwrap() {
            println!("{}", self.text.len());
            return self.text[new_pos];
        }
        return '\0';
    }

    fn eat(&mut self, offset: Option<usize>) {
        self.pos += 1 + offset.unwrap_or(0);
        self.col += 1 + offset.unwrap_or(0);
    }

    pub fn tokenise(&mut self) -> Vec<Token> {
        println!("{}", self.get_char(None));
        println!("{}", self.pos);
        while self.pos < self.text.len() {
            match self.get_char(None) {
                '\0' => {
                    self.consume(TokType::EOF, "<EOF>");
                }

                ' ' => (),
                '\t' => {
                    self.eat(None);
                }

                '\n' => {
                    self.pos += 1;
                    self.line += 1;
                    self.col = 0;
                }

                ',' => self.consume(TokType::SymbolComma, ","),

                '=' => {
                    if self.get_char(Some(1)) == '=' {
                        self.consume(TokType::OpEqEq, "==");
                    } else if self.get_char(Some(1)) == '>' {
                        self.consume(TokType::OpArrow, "=>")
                    } else {
                        self.consume(TokType::OpEq, "=")
                    }
                }

                _ => {
                    Report::build(ReportKind::Error, (), self.col)
                        .with_message("Unknown token")
                        .with_label(
                            Label::new(self.col..self.col)
                                .with_message("On this char")
                                .with_color(Color::Red),
                        )
                        .finish()
                        .print(Source::from(self.text.iter().collect::<String>()))
                        .unwrap();
                }
            }
        }

        self.tokens.clone()
    }
}
