#[derive(Debug, Clone, Copy)]
pub enum Token {
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
    KWordContinue, // Symbol: continue
}

pub fn tokenise(code: &str) -> Vec<Token> {
    let mut ret = Vec::new();
    
    ret
}
