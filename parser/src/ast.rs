use lexer::token::{Token, TokenError, TokenKind};

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
pub enum Node {
    Program(Program),
    ClassDecl(ClassDecl),
    MethodDecl(MethodDecl),
    VarDeclList(VarDeclList),
    VarDecl(VarDecl),
    Expr(Expr),
    Statement(Statement),
    Id(Id),
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
    pub body: Block,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub init: Option<Box<Expr>>, // Initializer expression
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
    pub body: Block,
    // TODO: remove this token field, implement NodeToken
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegularMethodDecl {
    pub ty: Box<Type>,
    pub name: Box<Id>,
    pub param_list: Box<ParamList>,
    pub body: Block,
    // TODO: remove this token field, implement NodeToken
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Statement>,
    // TODO: remove this token field
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Block),
    VarDeclList(VarDeclList),
    VarDecl(VarDecl),
    Print(Print),
    Expr(Box<Expr>),
    Break(Token),
    Return(Return),
    If(If),
    While(While),
    For(For),
    Assert(Assert),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assert {
    pub token: Token,
    pub cond: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub token: Token,
    pub cond: Expr,
    pub then: Box<Statement>,
    pub elze: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub token: Token,
    pub cond: Expr,
    pub block: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub token: Token,
    // NOTE: this can be either an `Expr`, `VarDecl` or `VarDeclList`
    pub init: Box<Node>,
    pub cond: Option<Expr>,
    pub tick: Option<Expr>,
    pub block: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    pub item: Box<Node>,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub expr: Option<Expr>,
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
    StringLiteral(Token),
    True(Token),
    False(Token),
    This(Token),
    Identifier(Id),
    New {
        token: Token,
        ty: Type,
    },
    Unary {
        op: Token,
        operand: Box<Node>,
    },
    Binary {
        /// Binary expression's first token (can be either `left.token()` or an opening paren)
        token: Token,
        op: Token,
        // TODO: Should this be a `Node` or can it be `Expr`?
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FieldAccess {
        // TODO: Should this be a `Node` or can it be `Expr`?
        object: Box<Expr>,
        field: Id,
    },
    MethodCall {
        object: Box<Expr>,
        name: Id,
        args: Vec<Expr>,
    },
    Assignment {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
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

impl NodeToken for Node {
    #[inline]
    fn token(&self) -> Token {
        match &self {
            Node::Program(_program) => todo!(),
            Node::ClassDecl(node) => node.token,
            Node::MethodDecl(node) => match node {
                MethodDecl::Main(decl) => decl.token,
                MethodDecl::Regular(decl) => decl.token,
            },
            Node::VarDeclList(node) => node.decls[0].token(),
            Node::VarDecl(node) => node.ty.token,
            Node::Expr(node) => node.token(),
            Node::Statement(_node) => todo!(),
            Node::Id(node) => node.0,
        }
    }
}

impl NodeToken for VarDecl {
    fn token(&self) -> Token {
        self.name.0
    }
}

impl NodeToken for Expr {
    fn token(&self) -> Token {
        match self {
            Expr::IntLiteral(token)
            | Expr::CharLiteral(token)
            | Expr::StringLiteral(token)
            | Expr::False(token)
            | Expr::True(token)
            | Expr::Identifier(Id(token))
            | Expr::New { token, .. }
            | Expr::Unary { op: token, .. }
            | Expr::Binary { op: token, .. }
            | Expr::This(token) => *token,
            Expr::FieldAccess { .. } => todo!(),
            Expr::MethodCall { object, .. } => object.token(),
            Expr::Assignment { lhs, .. } => lhs.token(),
        }
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
        match &self {
            Node::Program(program) => {
                let mut result = "Program:\n".to_string();
                for class in &program.classes {
                    result.push_str(&class.show(input, indent + Self::TAB));
                }
                result
            }
            Node::ClassDecl(class_decl) => class_decl.show(input, indent),
            Node::VarDecl(var_decl) => var_decl.show(input, indent),
            Node::VarDeclList(var_decl) => var_decl.show(input, indent),
            Node::MethodDecl(method_decl) => match method_decl {
                MethodDecl::Main(main) => main.show(input, indent + Self::TAB),
                MethodDecl::Regular(method) => method.show(input, indent + Self::TAB),
            },
            Node::Expr(expr) => expr.show(input, indent),
            _ => format!("no formatting for {self:?})"),
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
        format!(
            "{}MethodDecl: ID(name={}) {}\n{}{}{}",
            Self::indent(indent),
            self.name.0.value(input),
            self.token.formatted_pos(),
            self.ty.show(input, indent + Self::TAB),
            self.param_list.show(input, indent + Self::TAB),
            self.body.show(input, indent + Self::TAB),
        )
    }
}

impl<'src> Show<'src> for Assert {
    fn show(&self, input: &'src str, indent: usize) -> String {
        format!(
            "{}Assert: {}\n{}",
            Self::indent(indent),
            self.token.formatted_pos(),
            self.cond.show(input, indent + Self::TAB)
        )
    }
}

impl<'src> Show<'src> for Block {
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

impl<'src> Show<'src> for If {
    fn show(&self, input: &'src str, indent: usize) -> String {
        format!(
            "{}If: {}\n{}{}{}",
            Self::indent(indent),
            self.token.formatted_pos(),
            self.cond.show(input, indent + Self::TAB),
            self.then.show(input, indent + Self::TAB),
            match &self.elze {
                Some(stmt) => stmt.show(input, indent + Self::TAB),
                None => "".to_string(),
            }
        )
    }
}

impl<'src> Show<'src> for While {
    fn show(&self, input: &'src str, indent: usize) -> String {
        format!(
            "{}While: {}\n{}{}",
            Self::indent(indent),
            self.token.formatted_pos(),
            self.cond.show(input, indent + Self::TAB),
            self.block.show(input, indent + Self::TAB),
        )
    }
}

impl<'src> Show<'src> for For {
    fn show(&self, input: &'src str, indent: usize) -> String {
        // NOTE: DeclLists are formatted differently inside for statements
        let show_decl_list_inside_for = |decl_list: &Box<Node>| -> String {
            format!(
                "{}DeclList: {}\n{}",
                Self::indent(indent + Self::TAB),
                self.token.formatted_pos(),
                match decl_list.as_ref() {
                    Node::VarDeclList(var_decl_list) => var_decl_list,
                    _ => unreachable!(),
                }
                .decls
                .iter()
                .map(|decl| decl.show(input, indent + 2 * Self::TAB))
                .collect::<String>()
            )
        };

        format!(
            "{}For: {}\n{}{}{}{}",
            Self::indent(indent),
            self.token.formatted_pos(),
            show_decl_list_inside_for(&self.init),
            self.cond
                .iter()
                .map(|cond| cond.show(input, indent + Self::TAB))
                .collect::<String>(),
            self.tick
                .iter()
                .map(|tick| tick.show(input, indent + Self::TAB))
                .collect::<String>(),
            self.block.show(input, indent + Self::TAB),
        )
    }
}

impl<'src> Show<'src> for Statement {
    fn show(&self, input: &'src str, indent: usize) -> String {
        match self {
            Statement::Assert(assert) => assert.show(input, indent),
            Statement::Block(block) => block.show(input, indent),
            Statement::VarDecl(var_decl) => var_decl.show(input, indent),
            Statement::VarDeclList(node) => node.show(input, indent),
            Statement::Print(node) => {
                println!("{node:?}");
                let mut result = format!(
                    "{}Print: {}\n",
                    Self::indent(indent),
                    node.token.formatted_pos()
                );

                result.push_str(&node.item.show(input, indent + Self::TAB));
                result
            }
            Statement::Break(token) => {
                format!("{}Break: {}\n", Self::indent(indent), token.formatted_pos())
            }
            Statement::Return(retorn) => {
                let expr = match &retorn.expr {
                    Some(expr) => expr.show(input, indent + Self::TAB),
                    None => "".to_string(),
                };
                format!(
                    "{}Return: {}\n{}",
                    Self::indent(indent),
                    retorn.token.formatted_pos(),
                    expr
                )
            }
            Statement::Expr(expr) => expr.show(input, indent),
            Statement::If(node) => node.show(input, indent),
            Statement::While(node) => node.show(input, indent),
            Statement::For(node) => node.show(input, indent),
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

impl<'src> Show<'src> for Expr {
    fn show(&self, input: &'src str, indent: usize) -> String {
        match self {
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
            Expr::StringLiteral(tok) => {
                format!(
                    "{}Constant: String, {} {}\n",
                    Self::indent(indent),
                    tok.value(input),
                    tok.formatted_pos(),
                )
            }
            Expr::True(tok) => format!("{}True", Self::indent(indent)),
            Expr::False(_) => "False".to_string(),
            Expr::This(token) => {
                format!("{}This: {}\n", Self::indent(indent), token.formatted_pos())
            }
            Expr::Identifier(id) => id.show(input, indent),
            Expr::New { token, ty } => format!(
                "{}NewObject: {}\n{}",
                Self::indent(indent),
                token.formatted_pos(),
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
            Expr::Binary {
                token,
                op,
                left,
                right,
            } => {
                format!(
                    "{}BinaryOp: {} {}\n{}{}",
                    Self::indent(indent),
                    op.value(input),
                    token.formatted_pos(),
                    left.show(input, indent + Self::TAB),
                    right.show(input, indent + Self::TAB),
                )
            }
            Expr::FieldAccess { object, field } => {
                format!(
                    "{}FieldAccess: {}\n{}{}",
                    Self::indent(indent),
                    object.token().formatted_pos(),
                    object.show(input, indent + Self::TAB),
                    field.show(input, indent + Self::TAB)
                )
            }
            Expr::MethodCall { object, name, args } => {
                format!(
                    "{}MethodCall: {}\n{}{}{}",
                    Self::indent(indent),
                    object.token().formatted_pos(),
                    object.show(input, indent + Self::TAB),
                    name.show(input, indent + Self::TAB),
                    args.iter()
                        .map(|arg| arg.show(input, indent + Self::TAB))
                        .collect::<String>()
                )
            }
            Expr::Assignment { lhs, rhs } => {
                format!(
                    "{}Assignment: = {}\n{}{}",
                    Self::indent(indent),
                    lhs.token().formatted_pos(),
                    lhs.show(input, indent + Self::TAB),
                    rhs.show(input, indent + Self::TAB)
                )
            }
        }
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
        if !self.params.is_empty() {
            let mut result = String::new();
            result.push_str(&format!("{}ParamList:\n", Self::indent(indent)));
            result.push_str(
                &self
                    .params
                    .iter()
                    .map(|(ty, id)| {
                        format!(
                            "{}ParamDecl: ID(name={}) {}\n{}",
                            Self::indent(indent + Self::TAB),
                            id.0.value(input),
                            ty.token.formatted_pos(),
                            ty.show(input, indent + 2 * Self::TAB)
                        )
                    })
                    .collect::<String>(),
            );
            result
        } else {
            String::new()
        }
    }
}

impl Expr {
    /// Updates `self`'s `token` field.
    ///
    /// This is mainly used for parsing expressions inside parentheses, to make the inside
    /// expression's token point to the first opening paren.
    pub fn update_token(self, token: Token) -> Self {
        match self {
            Expr::Binary {
                token: _old_token,
                op,
                left,
                right,
            } => Expr::Binary {
                token,
                op,
                left,
                right,
            },
            _ => unreachable!(),
        }
    }
}

impl From<TokenError> for NodeErr {
    fn from(value: TokenError) -> Self {
        Self::LexErr(value)
    }
}
