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
    Program(Program),
    ClassDecl(ClassDecl),
    MethodDecl(MethodDecl),
    VarDecl(VarDecl),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    classes: Vec<ClassDecl>,
}

#[derive(Debug, PartialEq)]
pub struct ClassDecl {
    name: Box<Id>,
    // TODO: add extends
    var_decls: Vec<VarDecl>,
    method_decls: Vec<MethodDecl>,
    body: Compound,
}

#[derive(Debug, PartialEq)]
pub struct Id {}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub ty: TypeKind,
}

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Boolean,
    Char,
    Int,
    String,
    Custom,
}

#[derive(Debug, PartialEq)]
pub struct VarDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    // TODO: init field type
}

#[derive(Debug, PartialEq)]
pub struct MethodDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub param_list: Box<ParamList>,
}

#[derive(Debug, PartialEq)]
pub struct Compound {
    stmts: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {}

#[derive(Debug, PartialEq)]
pub struct ParamList {}

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

pub trait Show<'src> {
    fn show(&self, input: &'src str) -> String;
}

impl<'src> Show<'src> for Node {
    fn show(&self, input: &'src str) -> String {
        todo!()
    }
}
