#[derive(Debug, Clone, Copy)]
pub enum TokType {
    Ident,

    // Literal values
    ValNum,
    ValFloat, // TODO: can contain "E" and other shit
    ValBool,
    ValString,
    ValChar,

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
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub tok: TokType,
    pub val: String,
    pub start: usize,
    pub end: usize,
}

pub fn tokenise(src: String) -> Vec<Token> {
    let mut ret = Vec::new();
    let mut buf = Buffer::new(&src);
    let mut line = 0;
    while buf.in_bounds() {
        let data = buf.current("");
        if data == ' ' {
            buf.advance();
            continue;
        } else if data == '\n' {
            line += 1;
        } else if data == '@' {
            ret.push(Token {
                line,
                tok: TokType::SymbolAt,
                val: "@".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '.' {
            ret.push(Token {
                line,
                tok: TokType::SymbolDot,
                val: ".".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '#' {
            ret.push(Token {
                line,
                tok: TokType::SymbolHash,
                val: "#".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == ',' {
            ret.push(Token {
                line,
                tok: TokType::SymbolComma,
                val: ",".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == ':' {
            let start = buf.line_pos();
            buf.advance();
            if buf.in_bounds() && buf.current("") == ':' {
                ret.push(Token {
                    line,
                    tok: TokType::SymbolColcol,
                    val: "::".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::SymbolColon,
                    val: ":".to_string(),
                    start,
                    end: start,
                });
            }
        } else if data == '$' {
            ret.push(Token {
                line,
                tok: TokType::SymbolDollar,
                val: "$".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '?' {
            ret.push(Token {
                line,
                tok: TokType::SymbolQuestion,
                val: "?".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == ';' {
            ret.push(Token {
                line,
                tok: TokType::SymbolSemiColon,
                val: ";".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '{' {
            ret.push(Token {
                line,
                tok: TokType::BracketLCurly,
                val: "{".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '}' {
            ret.push(Token {
                line,
                tok: TokType::BracketRCurly,
                val: "}".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '(' {
            ret.push(Token {
                line,
                tok: TokType::BracketLParen,
                val: "(".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == ')' {
            ret.push(Token {
                line,
                tok: TokType::BracketRParen,
                val: ")".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '[' {
            ret.push(Token {
                line,
                tok: TokType::BracketLSquared,
                val: "[".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == ']' {
            ret.push(Token {
                line,
                tok: TokType::BracketRSquared,
                val: "]".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '*' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpMulEq,
                    val: "*=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpMul,
                    val: "*".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '/' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpDivEq,
                    val: "/=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpDiv,
                    val: "/".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '%' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpModEq,
                    val: "/=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpMod,
                    val: "/".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '+' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpPlusEq,
                    val: "+=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpPlus,
                    val: "+".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '-' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpMinusEq,
                    val: "-=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpMinus,
                    val: "-".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '>' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpGtEq,
                    val: ">=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                if buf.current("") == '>' {
                    // >> derivatives
                    let start = buf.line_pos();
                    buf.advance();
                    if buf.current("") == '=' {
                        ret.push(Token {
                            line,
                            tok: TokType::OpBitRshEq,
                            val: ">>=".to_string(),
                            start,
                            end: buf.line_pos(),
                        });
                    } else {
                        buf.back();
                        ret.push(Token {
                            line,
                            tok: TokType::OpBitRsh,
                            val: ">>".to_string(),
                            start,
                            end: buf.line_pos(),
                        });
                    }
                } else {
                    buf.back();
                    ret.push(Token {
                        line,
                        tok: TokType::OpGt,
                        val: ">".to_string(),
                        start: buf.line_pos(),
                        end: buf.line_pos(),
                    });
                }
            }
        } else if data == '<' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpLtEq,
                    val: "<=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                if buf.current("") == '>' {
                    // >> derivatives
                    let start = buf.line_pos();
                    buf.advance();
                    if buf.current("") == '=' {
                        ret.push(Token {
                            line,
                            tok: TokType::OpBitLshEq,
                            val: "<<=".to_string(),
                            start,
                            end: buf.line_pos(),
                        });
                    } else {
                        buf.back();
                        ret.push(Token {
                            line,
                            tok: TokType::OpBitLsh,
                            val: "<<".to_string(),
                            start,
                            end: buf.line_pos(),
                        });
                    }
                } else {
                    buf.back();
                    ret.push(Token {
                        line,
                        tok: TokType::OpLt,
                        val: "<".to_string(),
                        start: buf.line_pos(),
                        end: buf.line_pos(),
                    });
                }
            }
        } else if data == '!' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpNotEq,
                    val: "!=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpNot,
                    val: "!".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else if data == '=' {
            let start = buf.line_pos();
            buf.advance();
            match buf.current("") {
                '=' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpEqEq,
                        val: "==".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                '>' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpArrow,
                        val: "=>".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                _ => {
                    buf.back();
                    ret.push(Token {
                        line,
                        tok: TokType::OpEq,
                        val: "=".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
            }
        } else if data == '&' {
            let start = buf.line_pos();
            buf.advance();
            match buf.current("") {
                '=' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpBitAndEq,
                        val: "&=".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                '&' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpAnd,
                        val: "&&".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                _ => {
                    buf.back();
                    ret.push(Token {
                        line,
                        tok: TokType::OpBitAnd,
                        val: "&".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
            }
        } else if data == '|' {
            let start = buf.line_pos();
            buf.advance();
            match buf.current("") {
                '=' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpBitOrEq,
                        val: "|=".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                '|' => {
                    ret.push(Token {
                        line,
                        tok: TokType::OpOr,
                        val: "||".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
                _ => {
                    buf.back();
                    ret.push(Token {
                        line,
                        tok: TokType::OpBitOr,
                        val: "|".to_string(),
                        start,
                        end: buf.line_pos(),
                    });
                }
            }
        } else if data == '~' {
            ret.push(Token {
                line,
                tok: TokType::OpBitNot,
                val: "~".to_string(),
                start: buf.line_pos(),
                end: buf.line_pos(),
            });
        } else if data == '^' {
            let start = buf.line_pos();
            buf.advance();
            if buf.current("") == '=' {
                ret.push(Token {
                    line,
                    tok: TokType::OpBitXorEq,
                    val: "^=".to_string(),
                    start,
                    end: buf.line_pos(),
                });
            } else {
                buf.back();
                ret.push(Token {
                    line,
                    tok: TokType::OpBitXor,
                    val: "^".to_string(),
                    start: buf.line_pos(),
                    end: buf.line_pos(),
                });
            }
        } else {
            let start = buf.line_pos();
            let word = buf.get_word();
            match word.as_str() {
                "if" => ret.push(Token {
                    line,
                    tok: TokType::KWordIf,
                    val: "if".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "var" => ret.push(Token {
                    line,
                    tok: TokType::KWordVar,
                    val: "var".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "new" => ret.push(Token {
                    line,
                    tok: TokType::KWordNew,
                    val: "new".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "for" => ret.push(Token {
                    line,
                    tok: TokType::KWordFor,
                    val: "for".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "enum" => ret.push(Token {
                    line,
                    tok: TokType::KWordEnum,
                    val: "enum".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "func" => ret.push(Token {
                    line,
                    tok: TokType::KWordFunc,
                    val: "func".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "operator" => ret.push(Token {
                    line,
                    tok: TokType::KWordOperator,
                    val: "operator".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "else" => ret.push(Token {
                    line,
                    tok: TokType::KWordElse,
                    val: "else".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "case" => ret.push(Token {
                    line,
                    tok: TokType::KWordCase,
                    val: "case".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "break" => ret.push(Token {
                    line,
                    tok: TokType::KWordBreak,
                    val: "break".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "const" => ret.push(Token {
                    line,
                    tok: TokType::KWordConst,
                    val: "const".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "super" => ret.push(Token {
                    line,
                    tok: TokType::KWordSuper,
                    val: "super".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "while" => ret.push(Token {
                    line,
                    tok: TokType::KWordWhile,
                    val: "while".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "extern" => ret.push(Token {
                    line,
                    tok: TokType::KWordExtern,
                    val: "extern".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "virtual" => ret.push(Token {
                    line,
                    tok: TokType::KWordVirtual,
                    val: "virtual".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "class" => ret.push(Token {
                    line,
                    tok: TokType::KWordClass,
                    val: "class".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "as" => ret.push(Token {
                    line,
                    tok: TokType::KWordAs,
                    val: "as".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "import" => ret.push(Token {
                    line,
                    tok: TokType::KWordImport,
                    val: "import".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "pub" => ret.push(Token {
                    line,
                    tok: TokType::KWordPub,
                    val: "pub".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "switch" => ret.push(Token {
                    line,
                    tok: TokType::KWordSwitch,
                    val: "if".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "static" => ret.push(Token {
                    line,
                    tok: TokType::KWordStatic,
                    val: "static".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "return" => ret.push(Token {
                    line,
                    tok: TokType::KWordReturn,
                    val: "return".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "private" => ret.push(Token {
                    line,
                    tok: TokType::KWordPrivate,
                    val: "private".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "default" => ret.push(Token {
                    line,
                    tok: TokType::KWordDefault,
                    val: "default".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "try" => ret.push(Token {
                    line,
                    tok: TokType::KWordTry,
                    val: "try".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "catch" => ret.push(Token {
                    line,
                    tok: TokType::KWordCatch,
                    val: "catch".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "continue" => ret.push(Token {
                    line,
                    tok: TokType::KWordContinue,
                    val: "continue".to_string(),
                    start,
                    end: buf.line_pos(),
                }),
                "true" | "false" => ret.push(Token {
                    line,
                    tok: TokType::ValBool,
                    val: word,
                    start,
                    end: buf.line_pos(),
                }),
                _ => {
                    if is_num(&word) {
                        ret.push(Token {
                            line,
                            tok: TokType::ValNum,
                            val: word,
                            start,
                            end: buf.line_pos(),
                        });
                    } else if is_float(&word) {
                        ret.push(Token {
                            line,
                            tok: TokType::ValFloat,
                            val: word,
                            start,
                            end: buf.line_pos(),
                        });
                    } else if is_string(&word) {
                        ret.push(Token {
                            line,
                            tok: TokType::ValString,
                            val: word,
                            start,
                            end: buf.line_pos(),
                        });
                    } else if is_char(&word) {
                        ret.push(Token {
                            line,
                            tok: TokType::ValChar,
                            val: word,
                            start,
                            end: buf.line_pos(),
                        });
                    } else {
                        ret.push(Token {
                            line,
                            tok: TokType::Ident,
                            val: word,
                            start,
                            end: buf.line_pos(),
                        });
                    }
                }
            }
        }
        buf.advance();
    }
    ret
}

struct Buffer {
    data: String,
    index: usize,
    line_pos: usize,
}

impl Buffer {
    pub fn new(src: &String) -> Buffer {
        Buffer {
            data: src.clone(),
            index: 0,
            line_pos: 0,
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.index < self.data.len()
    }

    pub fn advance(&mut self) {
        if self.data.chars().nth(self.index) == Some('\n') {
            self.line_pos = 0;
        } else {
            self.line_pos += 1;
        }
        self.index += 1
    }

    pub fn current(&self, err: &str) -> char {
        self.data.chars().nth(self.index).expect(err)
    }

    pub fn line_pos(&self) -> usize {
        self.line_pos
    }

    pub fn back(&mut self) {
        self.index -= 1;
    }

    pub fn get_word(&mut self) -> String {
        let mut ret = String::new();
        let mut is_in_str = false;
        while self.in_bounds() && (!self.current("").is_whitespace() | is_in_str) {
            if self.current("") == '"' || self.current("") == '\'' {
                is_in_str = !is_in_str;
            }
            ret += &(self.current("").to_string());
            self.advance();
        }
        ret
    }
}

fn is_num(str: &String) -> bool {
    let mut ret = true;
    for chr in str.chars() {
        ret = chr.is_numeric();
    }
    ret
}

fn is_float(str: &String) -> bool {
    let mut ret = true;
    let mut dot = false;
    for chr in str.chars() {
        if !chr.is_numeric() {
            if chr == '.' && dot {
                dot = true;
                continue;
            }
            ret = false;
        }
    }
    ret
}

fn is_string(str: &String) -> bool {
    let v = str.chars().collect::<Vec<char>>();
    if v[0] != '"' || v[v.len() - 1] != '"' {
        return false;
    }
    let mut ctr = 0;
    for c in v {
        if c == '"' {
            ctr += 1;
        }
    }
    ctr == 2
}
fn is_char(str: &String) -> bool {
    let v = str.chars().collect::<Vec<char>>();
    if v.len() != 3 {
        return false;
    }
    if v[0] != '\'' || v[2] != '\'' {
        return false;
    }
    return true;
}
