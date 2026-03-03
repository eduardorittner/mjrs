use std::{
    fmt::{Display, Write},
    ops::Range,
};

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub kind: TokenKind,
    // Note: we don't use `Range<usize>` since it doesn't implement `Copy`
    pub range: (usize, usize),
    pub line: usize,
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
    /// '.'
    Dot,
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
    For,
    Main,
    Class,
    New,
    Return,
    Public,
    Private,
    Static,
    This,
    Print,
    // Types
    String,
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

/// Represents an error
#[derive(Clone, Debug, Copy)]
pub struct TokenError {
    pub c: char,
    /// Byte offset within the line
    pub offset: usize,
    pub line: usize,
}

pub type TokenResult = Result<Token, TokenError>;

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

pub fn fmt_tokens(tokens: &[TokenResult], src: &str) -> String {
    let mut result = String::new();
    let mut print_token = |tok: TokenResult| match tok {
        Ok(tok) => {
            writeln!(
                &mut result,
                "LexToken({},'{}',{},{})",
                tok.kind,
                &src[tok.range()],
                tok.line,
                tok.range.0
            )
            .unwrap();
        }
        Err(e) => {
            writeln!(
                &mut result,
                "Lexical error: Illegal character '{}' at {}:{}",
                e.c, e.line, e.offset
            )
            .unwrap();
        }
    };

    tokens.iter().for_each(|t| print_token(*t));

    result
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
                TokenKind::Greater => "GT",
                TokenKind::Less => "LT",
                TokenKind::GreaterEq => "GREATEREQ",
                TokenKind::LessEq => "LE",
                TokenKind::And => "AND",
                TokenKind::Or => "OR",
                TokenKind::Not => "NOT",
                TokenKind::Comma => "COMMA",
                TokenKind::Semicolon => "SEMI",
                TokenKind::Dot => "DOT",
                TokenKind::LeftParen => "LPAREN",
                TokenKind::RightParen => "RPAREN",
                TokenKind::LeftBracket => "LBRACKET",
                TokenKind::RightBracket => "RBRACKET",
                TokenKind::LeftBrace => "LBRACE",
                TokenKind::RightBrace => "RBRACE",
                TokenKind::If => "IF",
                TokenKind::Else => "ELSE",
                TokenKind::While => "WHILE",
                TokenKind::For => "FOR",
                TokenKind::Main => "MAIN",
                TokenKind::Class => "CLASS",
                TokenKind::New => "NEW",
                TokenKind::Return => "RETURN",
                TokenKind::Public => "PUBLIC",
                TokenKind::Private => "PRIVATE",
                TokenKind::Static => "STATIC",
                TokenKind::This => "THIS",
                TokenKind::Print => "PRINT",
                TokenKind::String => "STRING",
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
