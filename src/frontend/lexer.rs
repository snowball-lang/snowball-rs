use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"\w+")]
    Ident,

    // Literal values
    #[regex(r"[0-9]+", priority = 2)]
    ValNum,
    #[regex(r"[+-]?\d+(\.\d+)", priority = 3)]
    ValFloat, // TODO: can contain "E" and other shit
    #[regex(r"true|false")]
    ValBool,
    #[regex("\"([^]]+)\"")]
    ValString,
    #[regex(r"'.'")]
    ValChar,

    #[token("@")]
    SymbolAt, // Symbol: @
    #[token(".")]
    SymbolDot, // Symbol: .
    #[token("#")]
    SymbolHash, // Symbol: #
    #[token(",")]
    SymbolComma, // Symbol: ,
    #[token(":")]
    SymbolColon, // Symbol: :
    #[token("::")]
    SymbolColcol, // Symbol: ::
    #[token("$")]
    SymbolDollar, // Symbol: $
    #[token("?")]
    SymbolQuestion, // Symbol: ?
    #[token(";")]
    SymbolSemiColon, // Symbol: ;

    // |- BRACKETS -|
    #[token("{")]
    BracketLCurly, // Symbol: {
    #[token("}")]
    BracketRCurly, // Symbol: }
    #[token("(")]
    BracketLParen, // Symbol: (
    #[token(")")]
    BracketRParen, // Symbol: )
    #[token("[")]
    BracketRSquared, // Symbol: [
    #[token("]")]
    BracketLSquared, // Symbol: ]
    #[token("*")]
    OpMul, // Symbol: *
    #[token("%")]
    OpMod, // Symbol: %
    #[token("/")]
    OpDiv, // Symbol: /
    #[token("+")]
    OpPlus, // Symbol: +
    #[token("-")]
    OpMinus, // Symbol: -

    // Double characters
    #[token("*=")]
    OpMulEq, // Symbol: *=
    #[token("/=")]
    OpDivEq, // Symbol: /=
    #[token("%=")]
    OpModEq, // Symbol: %=
    #[token("+=")]
    OpPlusEq, // Symbol: +=
    #[token("-=")]
    OpMinusEq, // Symbol: -=

    // Single character tokens
    #[token(">")]
    OpGt, // Symbol: >
    #[token("<")]
    OpLt, // Symbol: <

    // Double character tokens
    #[token("=>")]
    OpArrow, // Symbol: =>
    #[token("==")]
    OpEqEq, // Symbol: ==
    #[token(">=")]
    OpGtEq, // Symbol: >=
    #[token("<=")]
    OpLtEq, // Symbol: <=
    #[token("!=")]
    OpNotEq, // Symbol: !=

    // Single character tokens
    #[token("=")]
    OpEq, // Symbol: =
    #[token("!")]
    OpNot, // Symbol: !

    // Double character tokens
    #[token("&&")]
    OpAnd, // Symbol: &&
    #[token("||")]
    OpOr, // Symbol: ||

    // Bitwise operations
    #[token("~")]
    OpBitNot, // Symbol: ~
    #[token("|")]
    OpBitOr, // Symbol: |
    #[token("&")]
    OpBitAnd, // Symbol: &
    #[token("^")]
    OpBitXor, // Symbol: ^
    #[token("|=")]
    OpBitOrEq, // Symbol: |=
    #[token(">>")]
    OpBitRsh, // Symbol: >>
    #[token("<<")]
    OpBitLsh, // Symbol: <<
    #[token("&=")]
    OpBitAndEq, // Symbol: &=
    #[token("^=")]
    OpBitXorEq, // Symbol: ^=
    #[token(">>=")]
    OpBitRshEq, // Symbol: >>=
    #[token("<<=")]
    OpBitLshEq, // Symbol: <<=

    #[token("if")]
    KWordIf, // Symbol: if
    #[token("let")]
    KWordVar, // Symbol: var
    #[token("new")]
    KWordNew, // Symbol: new
    #[token("for")]
    KWordFor, // Symbol: for
    #[token("enum")]
    KWordEnum, // Symbol: enum
    #[token("func")]
    KWordFunc, // Symbol: fn
    #[token("operator")]
    KWordOperator, // Symbol: operator
    #[token("else")]
    KWordElse, // Symbol: else
    #[token("case")]
    KWordCase, // Symbol: case
    #[token("break")]
    KWordBreak, // Symbol: break
    #[token("const")]
    KWordConst, // Symbol: const
    #[token("super")]
    KWordSuper, // Symbol: super
    #[token("while")]
    KWordWhile, // Symbol: while
    #[token("extern")]
    KWordExtern, // Synbol: extern
    #[token("virtual")]
    KWordVirtual, // Synbol: virtual
    #[token("class")]
    KWordClass, // Symbol: class
    #[token("as")]
    KWordAs, // Symbol: as
    #[token("use")]
    KWordImport, // Symbol: use
    #[token("pub")]
    KWordPub, // Symbol: pub
    #[token("switch")]
    KWordSwitch, // Symbol: switch
    #[token("static")]
    KWordStatic, // Symbol: static
    #[token("return")]
    KWordReturn, // Symbol: return
    #[token("priv")]
    KWordPrivate, // Symbol: priv
    #[token("default")]
    KWordDefault, // Symbol: default
    #[token("try")]
    KWordTry, // Symbol: try
    #[token("catch")]
    KWordCatch, // Symbol: catch
    #[token("continue")]
    KWordContinue, // Symbol: continue
}
