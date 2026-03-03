use std::{fmt::Display, ops::Range};

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

pub fn print_tokens(tokens: &[Token], src: &str) {
    let print_token = |token: Token| {
        println!(
            "LexToken({}, '{}', {}, {})",
            token.kind,
            &src[token.range()],
            0,
            token.range.0
        );
    };

    tokens.iter().for_each(|t| print_token(*t));
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Plus => "PLUS",
                TokenKind::Minus => "MINUS",
                TokenKind::Star => "STAR",
                TokenKind::Slash => "SLASH",
                TokenKind::Eq => "ASSIGN",
                TokenKind::EqEq => "EQ",
                TokenKind::NotEq => "NOTEQ",
                TokenKind::Greater => "GREATER",
                TokenKind::Less => "LESS",
                TokenKind::GreaterEq => "GREATEREQ",
                TokenKind::LessEq => "LESSEQ",
                TokenKind::And => "AND",
                TokenKind::Or => "OR",
                TokenKind::Not => "NOT",
                TokenKind::Comma => "COMMA",
                TokenKind::Semicolon => "SEMI",
                TokenKind::LeftParen => "LPAREN",
                TokenKind::RightParen => "RTPAREN",
                TokenKind::LeftBracket => "LBRACKET",
                TokenKind::RightBracket => "RBRACKET",
                TokenKind::LeftBrace => "LBRACE",
                TokenKind::RightBrace => "RBRACE",
                TokenKind::If => "IF",
                TokenKind::Else => "ELSE",
                TokenKind::While => "WHILE",
                TokenKind::Class => "CLASS",
                TokenKind::New => "NEW",
                TokenKind::Return => "RETURN",
                TokenKind::Public => "PUBLIC",
                TokenKind::Private => "PRIVATE",
                TokenKind::This => "THIS",
                TokenKind::Int => "INT",
                TokenKind::Boolean => "BOOLEAN",
                TokenKind::Void => "VOID",
                TokenKind::Id => "ID",
                TokenKind::False => "FALSE",
                TokenKind::True => "TRUE",
                TokenKind::IntLiteral => "INT_LITERAL",
                TokenKind::CharLiteral => "CHARLITERAL",
                TokenKind::StringLiteral => "STRINGLITERAL",
            }
        )
    }
}
