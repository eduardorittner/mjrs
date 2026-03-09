use lexer::token::{Token, TokenError, TokenKind};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub token: Token,
}

// TODO: implement pretty printing for better error reporting
#[derive(Debug, PartialEq)]
pub enum NodeErr {
    Unexpected {
        expected: Vec<TokenKind>,
        actual: Token,
    },
    LexErr(TokenError),
}

pub type NodeResult = Result<Node, NodeErr>;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Type(Type),
    Identifier,
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Type {
    ty: PrimitiveType,
    array_suffix: Option<()>,
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Boolean,
    Char,
    Int,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral,
    CharLiteral,
    True,
    False,
    This,
    Identifier(Box<Node>),
    New(Box<Node>),
    Unary { operand: Box<Node> },
    Binary { left: Box<Node>, right: Box<Node> },
}
