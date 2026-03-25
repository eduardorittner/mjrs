use lexer::token::{Token, TokenError, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    // TODO: remove this field, in favor of having a `token` field only on nodes with direct
    // relationships to tokens. Maybe have a trait `TokenTrait` (horrible name) with a `token`
    // method which returns an `Option<Token>` (nodes like `Program` would return `None` since they
    // are not directly associated with any one specific token) or returns a `Token` by having
    // nodes with no token associated return their first associated token (or any other logic).
    // Whether to return `Option<Token>` or `Token` probably depends on what the test outputs
    // expect
    pub token: Token,
}

// TODO: implement pretty printing for better error reporting
#[derive(Debug, Clone, PartialEq)]
pub enum NodeErr {
    Unexpected {
        expected: Vec<TokenKind>,
        actual: Token,
        line: u32,
        file: &'static str,
    },
    Eof,
    LexErr(TokenError),
}

pub type ParseResult<T> = Result<T, NodeErr>;

pub type NodeResult = ParseResult<Node>;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Program(Program),
    ClassDecl(ClassDecl),
    MethodDecl(MethodDecl),
    VarDeclList(VarDeclList),
    VarDecl(VarDecl),
    Expr(Expr),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub classes: Vec<ClassDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: Box<Id>,
    pub token: Token,
    // TODO: add extends
    pub var_decls: Vec<VarDecl>,
    pub method_decls: Vec<MethodDecl>,
    pub body: Compound,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id(pub Token);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Type {
    pub ty: TypeKind,
    pub token: Token,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeKind {
    Void,
    Boolean,
    Char,
    Int,
    String,
    Custom,
}

impl TryFrom<Token> for TypeKind {
    type Error = NodeErr;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::String => Ok(TypeKind::String),
            TokenKind::Int => Ok(TypeKind::Int),
            TokenKind::Char => Ok(TypeKind::Char),
            TokenKind::Boolean => Ok(TypeKind::Boolean),
            TokenKind::Void => Ok(TypeKind::Void),
            TokenKind::Id => Ok(TypeKind::Custom),
            _ => panic!("token is not valid type specifier"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclList {
    pub decls: Vec<VarDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub init: Option<Box<Node>>, // Initializer expression
}

#[derive(Debug, PartialEq, Clone)]
pub enum MethodDecl {
    Main(MainMethodDecl),
    Regular(RegularMethodDecl),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MainMethodDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub param_list: Box<ParamList>,
    pub body: Compound,
    // TODO: remove this token field, implement NodeToken
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegularMethodDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub param_list: Box<ParamList>,
    pub body: Compound,
    // TODO: remove this token field, implement NodeToken
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Compound {
    pub stmts: Vec<Statement>,
    // TODO: remove this token field
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclList(VarDeclList),
    VarDecl(VarDecl),
    Print(Print),
    Expression(Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    pub item: Box<Node>,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParamList {
    pub params: Vec<(Type, Id)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntLiteral(Token),
    CharLiteral(Token),
    True(Token),
    False(Token),
    This(Token),
    Identifier(Id),
    New(Type),
    Unary {
        op: Token,
        operand: Box<Node>,
    },
    Binary {
        op: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    FieldAccess {
        object: Box<Node>,
        field: Id,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpr {}

// TODO: better name?
pub trait NodeToken {
    /// Returns the token associated with a node
    ///
    /// This trait is needed since some nodes do not have a `token` field (like `VarDeclList`), but
    /// the output still expects them to be associated with a node, so we use this trait to do
    /// that.
    fn token(&self) -> Token;
}

impl NodeToken for VarDecl {
    fn token(&self) -> Token {
        self.name.0
    }
}

/// Format trait for a generic node
// TODO: add requirement on `NodeToken` trait
pub trait Show<'src> {
    const TAB: usize = 4;

    fn indent(indent: usize) -> String {
        " ".repeat(indent)
    }

    fn show(&self, input: &'src str, indent: usize) -> String;
}

impl<'src> Show<'src> for Node {
    fn show(&self, input: &'src str, indent: usize) -> String {
        match &self.kind {
            NodeKind::Program(program) => {
                let mut result = "Program:\n".to_string();
                for class in &program.classes {
                    result.push_str(&class.show(input, indent + Self::TAB));
                }
                result
            }
            NodeKind::ClassDecl(class_decl) => class_decl.show(input, indent),
            NodeKind::VarDecl(var_decl) => var_decl.show(input, indent),
            NodeKind::MethodDecl(method_decl) => {
                match method_decl {
                    MethodDecl::Main(main) => main.show(input, indent + Self::TAB),
                    MethodDecl::Regular(method) => method.show(input, indent),
                }
                // let mut result = format!(
                //     "MethodDecl: @ {}:{}",
                //     self.token.line(),
                //     self.token.column(),
                // );
                //
                // result.push_str(&method_decl.param_list.show(input));
                //
                // result.push_str(&format!(
                //     "\n            ID: {} @ {}:{}",
                //     method_decl.param_list.show(input),
                //     method_decl
                //         .param_list
                //         .params
                //         .first()
                //         .map(|(_, id)| id.token.line())
                //         .unwrap_or(method_decl.name.token.line()),
                //     method_decl
                //         .param_list
                //         .params
                //         .first()
                //         .map(|(_, id)| id.token.column())
                //         .unwrap_or(method_decl.name.token.column())
                // ));
                //
                // result.push_str(&format!(
                //     "\n            Compound: @ {}:{}",
                //     method_decl
                //         .body
                //         .stmts
                //         .first()
                //         .map(|stmt| {
                //             match stmt {
                //                 Statement::VarDecl(var_decl) => var_decl.name.token.line(),
                //                 _ => method_decl.name.token.line(),
                //             }
                //         })
                //         .unwrap_or(method_decl.name.token.line()),
                //     method_decl
                //         .body
                //         .stmts
                //         .first()
                //         .map(|stmt| {
                //             match stmt {
                //                 Statement::VarDecl(var_decl) => var_decl.name.token.column(),
                //                 _ => method_decl.name.token.column(),
                //             }
                //         })
                //         .unwrap_or(method_decl.name.token.column())
                // ));
                //
                // for stmt in &method_decl.body.stmts {
                //     match stmt {
                //         Statement::VarDecl(var_decl) => {
                //             result.push_str(&format!("\n                {}", var_decl.show(input)));
                //         }
                //         Statement::Print(expr) => {
                //             result.push_str(&format!(
                //                 "\n                Print: @ {}:{}",
                //                 expr.token.line(),
                //                 expr.token.column()
                //             ));
                //         }
                //         Statement::Expression(expr) => {
                //             result.push_str(&format!("\n                {}", expr.show(input)));
                //         }
                //     }
                // }
                //
                // result
            }
            NodeKind::Expr(expr) => match expr {
                Expr::IntLiteral(tok) => format!(
                    "{}Constant: int, {} {}\n",
                    Self::indent(indent),
                    tok.value(input),
                    tok.formatted_pos()
                ),
                Expr::CharLiteral(tok) => {
                    format!(
                        "{}Constant: char, {} {}\n",
                        Self::indent(indent),
                        tok.value(input),
                        tok.formatted_pos(),
                    )
                }
                Expr::True(_) => "True".to_string(),
                Expr::False(_) => "False".to_string(),
                Expr::This(_) => "This".to_string(),
                Expr::Identifier(id) => {
                    format!(
                        "{}ID: {} {}\n",
                        Self::indent(indent),
                        id.0.value(input),
                        self.token.formatted_pos()
                    )
                }
                Expr::New(ty) => format!(
                    "{}NewObject: {}\n{}",
                    Self::indent(indent),
                    self.token.formatted_pos(),
                    ty.show(&input, indent + Self::TAB)
                ),
                Expr::Unary { op, operand } => {
                    format!(
                        "{}UnaryOp: {} {}\n{}",
                        Self::indent(indent),
                        op.value(input),
                        op.formatted_pos(),
                        operand.show(input, indent + Self::TAB)
                    )
                }
                Expr::Binary { op, left, right } => {
                    let op_str = match op.kind {
                        TokenKind::Plus => "+",
                        TokenKind::Minus => "-",
                        TokenKind::Star => "*",
                        TokenKind::Slash => "/",
                        TokenKind::Mod => "%",
                        TokenKind::And => "&&",
                        TokenKind::Or => "||",
                        TokenKind::EqEq => "==",
                        TokenKind::NotEq => "!=",
                        TokenKind::Greater => ">",
                        TokenKind::Less => "<",
                        TokenKind::GreaterEq => ">=",
                        TokenKind::LessEq => "<=",
                        _ => "?",
                    };
                    format!(
                        "BinaryOp: {} @ {}:{}",
                        op_str,
                        left.token.line(),
                        left.token.column()
                    )
                }
                Expr::FieldAccess { object, field } => {
                    format!(
                        "{}FieldAccess: {}\n{}{}",
                        Self::indent(indent),
                        object.token.formatted_pos(),
                        object.show(input, indent + Self::TAB),
                        field.show(input, indent + Self::TAB)
                    )
                }
            },
            _ => format!("Node({:?})", self.token),
        }
    }
}

impl<'src> Show<'src> for Program {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = "Program:\n".to_string();
        for class in &self.classes {
            result.push_str(&class.show(input, indent + Self::TAB));
        }
        result
    }
}

impl<'src> Show<'src> for ClassDecl {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = format!(
            "{}ClassDecl: ID(name={}) @ {}:{}\n",
            Self::indent(indent),
            self.name.0.value(input),
            self.token.line(),
            self.token.column()
        );
        for var_decl in &self.var_decls {
            result.push_str(&var_decl.show(input, indent + Self::TAB));
        }
        for method_decl in &self.method_decls {
            result.push_str(&method_decl.show(input, indent + Self::TAB))
        }
        result
    }
}

impl<'src> Show<'src> for MethodDecl {
    fn show(&self, input: &'src str, indent: usize) -> String {
        match self {
            MethodDecl::Main(decl) => decl.show(input, indent),
            MethodDecl::Regular(decl) => decl.show(input, indent),
        }
    }
}

impl<'src> Show<'src> for MainMethodDecl {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = format!(
            "{}MainMethodDecl: {}\n",
            Self::indent(indent),
            self.token.formatted_pos(),
        );

        assert!(self.param_list.params.len() == 1);

        let (_ty, id) = self.param_list.params.first().unwrap();
        result.push_str(&id.show(input, indent + Self::TAB));

        result.push_str(&self.body.show(input, indent + Self::TAB));

        result
    }
}

impl<'src> Show<'src> for RegularMethodDecl {
    fn show(&self, input: &'src str, indent: usize) -> String {
        todo!()
    }
}

impl<'src> Show<'src> for Compound {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = format!(
            "{}Compound: {}\n",
            Self::indent(indent),
            self.token.formatted_pos()
        );

        let body: String = self
            .stmts
            .iter()
            .map(|stmt| stmt.show(input, indent + Self::TAB))
            .collect();

        result.push_str(&body);

        result
    }
}

impl<'src> Show<'src> for Statement {
    fn show(&self, input: &'src str, indent: usize) -> String {
        match self {
            Statement::VarDecl(var_decl) => var_decl.show(input, indent),
            Statement::VarDeclList(node) => node.show(input, indent),
            Statement::Print(node) => {
                let mut result = format!(
                    "{}Print: {}\n",
                    Self::indent(indent),
                    node.token.formatted_pos()
                );

                result.push_str(&node.item.show(input, indent + Self::TAB));
                result
            }
            Statement::Expression(node) => todo!(),
        }
    }
}

impl<'src> Show<'src> for Id {
    fn show(&self, input: &'src str, indent: usize) -> String {
        format!(
            "{}ID: {} {}\n",
            Self::indent(indent),
            self.0.value(input),
            self.0.formatted_pos()
        )
    }
}

impl<'src> Show<'src> for Type {
    fn show(&self, input: &'src str, indent: usize) -> String {
        format!(
            "{}Type: {} {}\n",
            Self::indent(indent),
            match &self.ty {
                TypeKind::Void => "void".to_string(),
                TypeKind::Int => "int".to_string(),
                TypeKind::Char => "char".to_string(),
                TypeKind::Boolean => "boolean".to_string(),
                TypeKind::String => "String".to_string(),
                TypeKind::Custom => format!("ID(name={})", self.token.value(input).to_string()),
            },
            self.token.formatted_pos()
        )
    }
}

impl<'src> Show<'src> for VarDecl {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = format!(
            "{}VarDecl: ID(name={}) {}\n",
            Self::indent(indent),
            self.token().value(input),
            self.name.0.formatted_pos()
        );

        result.push_str(&self.ty.show(input, indent + Self::TAB));
        if let Some(init) = &self.init {
            result.push_str(&init.show(input, indent + Self::TAB));
        }

        result
    }
}

impl<'src> Show<'src> for VarDeclList {
    fn show(&self, input: &'src str, indent: usize) -> String {
        self.decls
            .iter()
            .map(|decl| decl.show(input, indent))
            .collect()
    }
}

impl<'src> Show<'src> for ParamList {
    fn show(&self, input: &'src str, indent: usize) -> String {
        let mut result = String::new();
        for (i, (ty, id)) in self.params.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{} {}", ty.show(input, indent), id.0.value(input)));
        }
        result
    }
}
