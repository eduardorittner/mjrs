use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenKind,
    // Note: we don't use `Range<usize>` since it doesn't implement `Copy`
    pub range: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    // Arithmetic ops
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    // Relational ops
    /// '='
    Eq,
    /// '=='
    EqEq,
    /// '!='
    NotEq,
    /// '>'
    Greater,
    /// '<'
    Less,
    /// '>='
    GreaterEq,
    /// '<='
    LessEq,
    // Logical ops
    /// '&&'
    And,
    /// '||'
    Or,
    /// '!'
    Not,
    // Punctuation
    /// ','
    Comma,
    /// ';'
    Semicolon,
    /// '('
    LeftParen,
    /// ')'
    RightParen,
    /// '['
    LeftBracket,
    /// ']'
    RightBracket,
    /// '{'
    LeftBrace,
    /// '}'
    RightBrace,
    // Keywords
    If,
    Else,
    While,
    Class,
    New,
    Return,
    Public,
    Private,
    This,
    // Types
    Int,
    Boolean,
    Void,
    // Literals
    Id,
    False,
    True,
    IntLiteral,
    CharLiteral,
    StringLiteral,
}

impl Token {
    /// Returns a standard `Range<usize>`. Useful for getting access to all common methods
    /// implemented on `Range<usize>` for array access, slicing and so on.
    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.range.0,
            end: self.range.1,
        }
    }
}
