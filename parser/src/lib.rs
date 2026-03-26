use lexer::token::{Token, TokenKind, TokenResult};

use crate::ast::{
    Assert, Block, Expr, ExprList, For, Id, InitList, MainMethodDecl, MethodDecl, Node, NodeErr,
    NodeResult, ParseResult, Print, RegularMethodDecl, Return, Statement, Type, TypeKind, VarDecl,
    VarDeclList, While,
};

pub mod ast;

pub struct Parser<'src> {
    /// Needed for reading the values from literals and identifiers
    input: &'src str,

    tokens: &'src Vec<TokenResult>,

    /// Current position inside `tokens`
    idx: usize,
}

macro_rules! advance {
    ($parser:expr, $kinds:expr) => {
        $parser.advance($kinds, line!(), file!())
    };
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str, tokens: &'src Vec<TokenResult>) -> Parser<'src> {
        Parser {
            input,
            tokens,
            idx: 0,
        }
    }

    /// Returns the root `Node` of the parsed ast
    pub fn parse(&mut self) -> NodeResult {
        self.program()
    }

    fn program(&mut self) -> NodeResult {
        // For now, let's just parse the first class
        let class_node = self.class_decl()?;

        // TODO: maybe make a macro which returns the type or errors?
        // instead of having to always match on returned nodes
        let class_decl = match class_node {
            Node::ClassDecl(decl) => decl,
            _ => panic!("Expected ClassDecl"),
        };
        Ok(Node::Program(crate::ast::Program {
            classes: vec![class_decl],
        }))
    }

    fn class_decl(&mut self) -> NodeResult {
        // Parse "class" keyword
        let class_token = advance!(self, &[TokenKind::Class])?;

        // Parse class name (identifier)
        let name_token = advance!(self, &[TokenKind::Id])?;
        let name = Id(name_token);

        // Parse "{"
        let compound_start = advance!(self, &[TokenKind::LeftBrace])?;

        // Parse class body
        let mut var_decls = Vec::new();
        let mut method_decls = Vec::new();

        // Parse variable and method declarations
        while let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::Int | TokenKind::Char | TokenKind::Boolean => {
                    // Check if it's a variable or method declaration
                    // For now, let's just parse variable declarations
                    let var_decl_node = self.var_decl_list(true)?;
                    let var_decl = match var_decl_node {
                        Node::VarDecl(decl) => decl,
                        _ => panic!("Expected VarDecl"),
                    };
                    var_decls.push(var_decl);
                }
                TokenKind::Public => {
                    // Method declaration
                    let method_decl_node = self.method_decl()?;
                    match method_decl_node {
                        Node::MethodDecl(decl) => method_decls.push(decl),
                        _ => panic!("Expected MethodDecl"),
                    };
                }
                TokenKind::RightBrace => {
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        // Parse "}"
        advance!(self, &[TokenKind::RightBrace])?;

        Ok(Node::ClassDecl(crate::ast::ClassDecl {
            name: Box::new(name),
            token: class_token,
            var_decls,
            method_decls,
            body: crate::ast::Block {
                stmts: vec![],
                token: compound_start,
            },
        }))
    }

    // NOTE: The tests treat variable declaration lists differently depending on context, most of
    // the times a list with only one declaration is unwrapped into a `VarDecl`, while in others
    // it's still kept as a `VarDeclList`
    fn var_decl_list(&mut self, unwrap_single_decl: bool) -> NodeResult {
        let mut decls = Vec::new();
        // Parse type
        let ty = self.type_specifier().ok_or(NodeErr::Eof)??;

        while let Ok(var_decl) = self.var_decl(ty) {
            decls.push(var_decl);

            if let Some(_comma) = self.advance_if(&[TokenKind::Comma]) {
                continue;
            }

            break;
        }

        // Parse ";"
        advance!(self, &[TokenKind::Semicolon])?;

        if decls.len() == 1 && unwrap_single_decl {
            Ok(Node::VarDecl(decls.into_iter().next().unwrap()))
        } else {
            Ok(Node::VarDeclList(VarDeclList {
                decls,
                token: ty.token,
            }))
        }
    }

    /// Parses a single variable declaration after a type_specifier
    fn var_decl(&mut self, ty: Type) -> ParseResult<VarDecl> {
        let name_token = advance!(self, &[TokenKind::Id])?;
        let name = Id(name_token);

        // Optional initializer expression
        let init = if let Some(_eq) = self.advance_if(&[TokenKind::Eq]) {
            // Initializer list
            if self.advance_if(&[TokenKind::LeftBrace]).is_some() {
                let mut items = Vec::new();

                loop {
                    items.push(self.expr()?);

                    if self.advance_if(&[TokenKind::Comma]).is_some() {
                        // support trailing comma
                        if self.advance_if(&[TokenKind::RightBrace]).is_some() {
                            break Some(Box::new(Node::InitList(InitList { items })));
                        } else {
                            continue;
                        }
                    } else {
                        advance!(self, &[TokenKind::RightBrace])?;
                        break Some(Box::new(Node::InitList(InitList { items })));
                    }
                }
            } else {
                // Regular expression
                Some(Box::new(Node::Expr(self.expr()?)))
            }
        } else {
            None
        };

        Ok(VarDecl {
            ty: Box::new(ty.clone()),
            name: Box::new(name),
            init: init,
        })
    }

    /// Consumes the next token, returning an error if the token kind is not present in `expected`
    fn advance(
        &mut self,
        expected: &[TokenKind],
        line: u32,
        file: &'static str,
    ) -> Result<Token, NodeErr> {
        match self.tokens[self.idx] {
            Ok(tok) if expected.contains(&tok.kind) => {
                self.idx += 1;
                Ok(tok)
            }
            Ok(tok) => {
                // TODO: should we  `self.idx` here?
                Err(NodeErr::Unexpected {
                    expected: Vec::from(expected),
                    actual: tok,
                    line,
                    file,
                })
            }
            Err(e) => Err(NodeErr::LexErr(e)),
        }
    }

    fn advance_if(&mut self, expected: &[TokenKind]) -> Option<Token> {
        match self.tokens[self.idx] {
            Ok(tok) if expected.contains(&tok.kind) => {
                self.idx += 1;
                Some(tok)
            }
            _ => None,
        }
    }

    /// Returns the next token, without consuming it
    fn peek(&mut self) -> Option<TokenResult> {
        if self.idx != self.tokens.len() {
            Some(self.tokens[self.idx])
        } else {
            None
        }
    }

    /// Returns the n-th next token, without consuming it
    ///
    /// NOTE: `peek_n(1)` is equivalent to `peek()`
    fn peek_n(&mut self, n: usize) -> Option<TokenResult> {
        let idx = self.idx + n - 1;
        if idx != self.tokens.len() {
            Some(self.tokens[idx])
        } else {
            None
        }
    }

    fn args(&mut self) -> ParseResult<ExprList> {
        let mut args = Vec::new();

        while let Some(expr) = self.expr_try() {
            args.push(expr);
            if self.advance_if(&[TokenKind::Comma]).is_none() {
                break;
            }
        }

        Ok(ExprList { exprs: args })
    }

    fn expr_try(&mut self) -> Option<Expr> {
        // NOTE: this is sort of hacky, I guess
        self.parse_expr(0).ok()
    }

    fn expr(&mut self) -> ParseResult<Expr> {
        self.parse_expr(0)
    }

    fn get_precedence(&self, op: TokenKind) -> u8 {
        match op {
            TokenKind::Or => 1,
            TokenKind::And => 2,
            TokenKind::EqEq | TokenKind::NotEq => 3,
            TokenKind::Less | TokenKind::LessEq | TokenKind::Greater | TokenKind::GreaterEq => 4,
            TokenKind::Plus | TokenKind::Minus => 5,
            TokenKind::Star | TokenKind::Slash | TokenKind::Mod => 6,
            _ => 0,
        }
    }

    fn parse_expr(&mut self, min_precedence: u8) -> ParseResult<Expr> {
        let first_token = self.peek().unwrap()?;
        let mut left = self.primary_expr()?;

        while let Some(Ok(token)) = self.peek() {
            if !token.kind.is_binary_operator() {
                // Assignment expression
                if token.kind == TokenKind::Eq {
                    advance!(self, &[TokenKind::Eq])?;
                    let right = self.expr()?;

                    left = Expr::Assignment {
                        lhs: Box::new(left),
                        rhs: Box::new(right),
                    };
                }

                break;
            }

            let precedence = self.get_precedence(token.kind);
            if precedence < min_precedence {
                break;
            }

            self.idx += 1; // consume operator

            // For right associative operators, use precedence
            // For left associative operators, use precedence + 1
            let next_precedence = precedence + 1;
            let right = self.parse_expr(next_precedence)?;

            left = Expr::Binary {
                token: first_token,
                op: token,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // TODO: this is currently parsing more than primary expressions, rename ir or refactor out
    /// Parses primary expressions, which are either literals or of the form '(' <expr> ')'
    fn primary_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary_expr_without_field_access()?;

        // TODO: cleanup postfix operator parsing

        // postfix operators
        while let Some(_dot_token) = self.advance_if(&[TokenKind::Dot]) {
            // Expect an identifier after the dot
            let field_token = advance!(self, &[TokenKind::Id])?;
            let field = Id(field_token);

            if self.advance_if(&[TokenKind::LeftParen]).is_some() {
                let args = self.args()?;
                advance!(self, &[TokenKind::RightParen])?;

                expr = Expr::MethodCall {
                    name: field,
                    object: Box::new(expr),
                    args: args.exprs,
                }
            } else {
                expr = Expr::FieldAccess {
                    object: Box::new(expr),
                    field: field,
                };
            }
        }

        if self.advance_if(&[TokenKind::LeftBracket]).is_some() {
            let idx = self.expr()?;

            advance!(self, &[TokenKind::RightBracket])?;

            expr = Expr::ArrayRef {
                object: Box::new(expr),
                idx: Box::new(idx),
            }
        }

        Ok(expr)
    }

    /// Parses primary expressions without field access
    fn primary_expr_without_field_access(&mut self) -> ParseResult<Expr> {
        if let Some(Ok(token)) = self.peek() {
            match token.kind {
                // Unary operators
                TokenKind::Minus | TokenKind::Not | TokenKind::Plus => {
                    self.idx += 1; // consume operator
                    let operand = self.primary_expr()?;
                    return Ok(Expr::Unary {
                        op: token,
                        operand: Box::new(Node::Expr(operand)),
                    });
                }
                TokenKind::True => {
                    self.idx += 1;
                    Ok(Expr::True(token))
                }
                TokenKind::False => {
                    self.idx += 1;
                    Ok(Expr::False(token))
                }
                TokenKind::CharLiteral => {
                    self.idx += 1;
                    Ok(Expr::CharLiteral(token))
                }
                TokenKind::This => {
                    self.idx += 1;
                    Ok(Expr::This(token))
                }
                TokenKind::Id => {
                    self.idx += 1;
                    Ok(Expr::Identifier(Id(token)))
                }
                TokenKind::New => {
                    self.idx += 1;

                    if let Some(Ok(id_token)) = self.identifier() {
                        advance!(self, &[TokenKind::LeftParen])?;
                        advance!(self, &[TokenKind::RightParen])?;
                        Ok(Expr::New {
                            token,
                            ty: Type {
                                ty: TypeKind::Custom,
                                token: id_token.0,
                            },
                        })
                    } else {
                        todo!()
                    }
                }
                TokenKind::StringLiteral => {
                    self.idx += 1;
                    Ok(Expr::StringLiteral(token))
                }
                TokenKind::IntLiteral => {
                    self.idx += 1;
                    Ok(Expr::IntLiteral(token))
                }
                TokenKind::LeftParen => {
                    self.idx += 1; // consume '('
                    let expr = self.expr()?;
                    advance!(self, &[TokenKind::RightParen])?; // consume ')'
                    // TODO: update expr.token here

                    Ok(expr)
                }
                _ => Err(NodeErr::Unexpected {
                    expected: vec![
                        TokenKind::True,
                        TokenKind::False,
                        TokenKind::IntLiteral,
                        TokenKind::CharLiteral,
                        TokenKind::This,
                        TokenKind::Id,
                        TokenKind::StringLiteral,
                        TokenKind::LeftParen,
                        TokenKind::Minus,
                        TokenKind::Not,
                    ],
                    actual: token,
                    line: line!(),
                    file: file!(),
                }),
            }
        } else {
            todo!()
        }
    }

    fn method_decl(&mut self) -> NodeResult {
        let public = advance!(self, &[TokenKind::Public])?;

        // Consume optional static keyword
        self.advance_if(&[TokenKind::Static]);

        let return_type = advance!(
            self,
            &[
                TokenKind::Void,
                TokenKind::Int,
                TokenKind::Char,
                TokenKind::Boolean,
            ]
        )?;

        let return_type = match return_type.kind {
            TokenKind::Void => crate::ast::Type {
                ty: crate::ast::TypeKind::Custom,
                token: return_type,
            },
            TokenKind::Int => crate::ast::Type {
                ty: crate::ast::TypeKind::Int,
                token: return_type,
            },
            TokenKind::Char => crate::ast::Type {
                ty: crate::ast::TypeKind::Char,
                token: return_type,
            },
            TokenKind::Boolean => crate::ast::Type {
                ty: crate::ast::TypeKind::Boolean,
                token: return_type,
            },
            _ => unreachable!(),
        };

        let name_token = advance!(self, &[TokenKind::Id, TokenKind::Main])?;
        let name = Id(name_token);

        advance!(self, &[TokenKind::LeftParen])?;

        let param_list = self.param_list();

        advance!(self, &[TokenKind::RightParen])?;

        let body = self.compound_stmt()?;

        Ok(if name_token.kind == TokenKind::Main {
            Node::MethodDecl(MethodDecl::Main(MainMethodDecl {
                ty: Box::new(return_type),
                name: Box::new(name),
                param_list: Box::new(param_list),
                body,
                token: public,
            }))
        } else {
            Node::MethodDecl(MethodDecl::Regular(RegularMethodDecl {
                ty: Box::new(return_type),
                name: Box::new(name),
                param_list: Box::new(param_list),
                body,
                token: public,
            }))
        })
    }

    fn param_list(&mut self) -> crate::ast::ParamList {
        // Parse parameter list: (Type Id, Type Id, ...)
        let mut params = Vec::new();

        // Check if there are any parameters
        if let Some(Ok(token)) = self.peek() {
            if token.kind != TokenKind::RightParen {
                // Parse first parameter
                let type_token = advance!(
                    self,
                    &[
                        TokenKind::Int,
                        TokenKind::Char,
                        TokenKind::Boolean,
                        TokenKind::String,
                    ]
                )
                .expect("Expected parameter type");

                // Check for array type
                let param_type = match type_token.kind {
                    TokenKind::Int => crate::ast::Type {
                        ty: crate::ast::TypeKind::Int,
                        token: type_token,
                    },
                    TokenKind::Char => crate::ast::Type {
                        ty: crate::ast::TypeKind::Char,
                        token: type_token,
                    },
                    TokenKind::Boolean => crate::ast::Type {
                        ty: crate::ast::TypeKind::Boolean,
                        token: type_token,
                    },
                    TokenKind::String => crate::ast::Type {
                        ty: crate::ast::TypeKind::String,
                        token: type_token,
                    },
                    _ => unreachable!(),
                };

                // Check for array brackets
                if let Some(Ok(bracket_token)) = self.peek() {
                    if bracket_token.kind == TokenKind::LeftBracket {
                        self.idx += 1; // consume '['
                        advance!(self, &[TokenKind::RightBracket]).expect("Expected ']'");
                        // For now, we'll just keep the base type and ignore the array part
                    }
                }

                let name_token = advance!(self, &[TokenKind::Id]).expect("Expected parameter name");
                let param_name = Id(name_token);

                params.push((param_type, param_name));

                // Parse additional parameters
                while let Some(Ok(comma_token)) = self.peek() {
                    if comma_token.kind == TokenKind::Comma {
                        self.idx += 1; // consume comma

                        let type_token = advance!(
                            self,
                            &[
                                TokenKind::Int,
                                TokenKind::Char,
                                TokenKind::Boolean,
                                TokenKind::String,
                            ]
                        )
                        .expect("Expected parameter type");

                        // Check for array type
                        let param_type = match type_token.kind {
                            TokenKind::Int => crate::ast::Type {
                                ty: crate::ast::TypeKind::Int,
                                token: type_token,
                            },
                            TokenKind::Char => crate::ast::Type {
                                ty: crate::ast::TypeKind::Char,
                                token: type_token,
                            },
                            TokenKind::Boolean => crate::ast::Type {
                                ty: crate::ast::TypeKind::Boolean,
                                token: type_token,
                            },
                            TokenKind::String => crate::ast::Type {
                                ty: crate::ast::TypeKind::String,
                                token: type_token,
                            },
                            _ => unreachable!(),
                        };

                        // Check for array brackets
                        if let Some(Ok(bracket_token)) = self.peek() {
                            if bracket_token.kind == TokenKind::LeftBracket {
                                self.idx += 1; // consume '['
                                advance!(self, &[TokenKind::RightBracket]).expect("Expected ']'");
                                // For now, we'll just keep the base type and ignore the array part
                            }
                        }

                        let name_token =
                            advance!(self, &[TokenKind::Id]).expect("Expected parameter name");
                        let param_name = Id(name_token);

                        params.push((param_type, param_name));
                    } else {
                        break;
                    }
                }
            }
        }

        crate::ast::ParamList { params }
    }

    fn stmt(&mut self) -> ParseResult<Statement> {
        if let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::Int | TokenKind::Char | TokenKind::Boolean => {
                    // Variable declaration
                    let var_decl_node = self
                        .var_decl_list(true)
                        .expect("Expected variable declaration");
                    match var_decl_node {
                        Node::VarDecl(decl) => Ok(Statement::VarDecl(decl)),
                        Node::VarDeclList(decl_list) => Ok(Statement::VarDeclList(decl_list)),
                        _ => panic!("Expected VarDecl"),
                    }
                }
                TokenKind::Print => {
                    // Print statement
                    let print = advance!(self, &[TokenKind::Print]).expect("checked before-hand");
                    advance!(self, &[TokenKind::LeftParen]).expect("Expected '('");
                    let args = self.args()?;
                    advance!(self, &[TokenKind::RightParen]).expect("Expected ')'");
                    advance!(self, &[TokenKind::Semicolon]).expect("Expected ';'");

                    Ok(Statement::Print(Print { args, token: print }))
                }
                TokenKind::Break => {
                    let break_token =
                        advance!(self, &[TokenKind::Break]).expect("checked before-hand");

                    advance!(self, &[TokenKind::Semicolon])
                        .expect("expected ';' after 'break' keyword");

                    Ok(Statement::Break(break_token))
                }
                TokenKind::Return => {
                    let return_token =
                        advance!(self, &[TokenKind::Return]).expect("checked before-hand");

                    let expr = self.expr_try();

                    advance!(self, &[TokenKind::Semicolon])
                        .expect("expected ';' after 'return' keyword and expression");

                    Ok(Statement::Return(Return {
                        token: return_token,
                        expr,
                    }))
                }
                TokenKind::Id => {
                    // Either expression statement or declaration
                    match self.peek_n(2) {
                        Some(Ok(t)) => match t.kind {
                            TokenKind::Id => {
                                let decl = self.var_decl_list(true)?;
                                let decl = match decl {
                                    Node::VarDecl(decl) => decl,
                                    _ => panic!("Expected VarDecl"),
                                };
                                Ok(Statement::VarDecl(decl))
                            }
                            _ => {
                                // Assume it's an expression statement
                                self.expr_stmt()
                            }
                        },
                        _ => unimplemented!(),
                    }
                }
                TokenKind::This => self.expr_stmt(),
                TokenKind::Assert => self.assert_stmt().map(|ok| Statement::Assert(ok)),
                TokenKind::If => self.if_stmt(),
                TokenKind::While => self.while_stmt(),
                TokenKind::For => self.for_stmt(),
                TokenKind::LeftBrace => Ok(Statement::Block(self.compound_stmt()?)),
                tok => panic!("{tok:?}"),
            }
        } else {
            Err(NodeErr::Eof)
        }
    }

    fn compound_stmt(&mut self) -> ParseResult<Block> {
        // Parse "{"
        let compound_start = advance!(self, &[TokenKind::LeftBrace])?;

        let mut stmts = Vec::new();

        // Parse statements until "}"
        while let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::RightBrace => {
                    break;
                }
                _tok => {
                    stmts.push(self.stmt()?);
                }
            }
        }

        // Parse "}"
        advance!(self, &[TokenKind::RightBrace]).expect("Expected '}' to end compound statement");

        Ok(Block {
            stmts,
            token: compound_start,
        })
    }

    fn assert_stmt(&mut self) -> ParseResult<Assert> {
        let token = advance!(self, &[TokenKind::Assert])?;
        let cond = self.expr()?;
        advance!(self, &[TokenKind::Semicolon])?;

        Ok(Assert { token, cond })
    }

    fn for_stmt(&mut self) -> ParseResult<Statement> {
        let token = advance!(self, &[TokenKind::For])?;

        advance!(self, &[TokenKind::LeftParen])?;

        let init = if self
            .peek()
            .is_some_and(|result| result.is_ok_and(|tok| TYPE_SPECIFIERS.contains(&tok.kind)))
            && self
                .peek_n(2)
                .is_some_and(|result| result.is_ok_and(|tok| tok.kind == TokenKind::Id))
        {
            // var_decl_list() consumes the ';'
            let mut init = self.var_decl_list(false)?;
            // NOTE: for some reason tests expect DeclList's token to be the same as the for statement
            match init {
                Node::VarDeclList(ref mut list) => list.token = token,
                _ => unreachable!(),
            };
            init
        } else {
            let expr = Node::Expr(self.expr()?);
            advance!(self, &[TokenKind::Semicolon])?;
            expr
        };

        let cond = self.expr_try();
        advance!(self, &[TokenKind::Semicolon])?;

        let tick = self.expr_try();

        advance!(self, &[TokenKind::RightParen])?;

        let block = self.stmt()?;

        Ok(Statement::For(For {
            token,
            init: Box::new(init),
            cond,
            tick,
            block: Box::new(block),
        }))
    }

    fn while_stmt(&mut self) -> ParseResult<Statement> {
        let while_token = advance!(self, &[TokenKind::While])?;

        advance!(self, &[TokenKind::LeftParen])?;
        let cond = self.expr()?;
        advance!(self, &[TokenKind::RightParen])?;

        let block = self.stmt()?;

        Ok(Statement::While(While {
            token: while_token,
            cond,
            block: Box::new(block),
        }))
    }

    // TODO: make this return `If`
    fn if_stmt(&mut self) -> ParseResult<Statement> {
        let if_token = advance!(self, &[TokenKind::If])?;

        advance!(self, &[TokenKind::LeftParen])?;
        let cond = self.expr()?;
        advance!(self, &[TokenKind::RightParen])?;

        let then = self.stmt()?;

        let elze = if let Some(_else_token) = self.advance_if(&[TokenKind::Else]) {
            Some(self.stmt()?)
        } else {
            None
        };

        Ok(Statement::If(ast::If {
            token: if_token,
            cond,
            then: Box::new(then),
            elze: elze.map(|stmt| Box::new(stmt)),
        }))
    }

    fn expr_stmt(&mut self) -> ParseResult<Statement> {
        let expr = self.expr()?;
        let stmt = Statement::Expr(Box::new(expr));
        advance!(self, &[TokenKind::Semicolon])?;

        Ok(stmt)
    }

    fn identifier(&mut self) -> Option<ParseResult<Id>> {
        if let Some(token) = self.advance_if(&[TokenKind::Id]) {
            Some(Ok(Id(token)))
        } else {
            None
        }
    }

    fn type_specifier(&mut self) -> Option<ParseResult<Type>> {
        if let Some(token) = self.advance_if(&TYPE_SPECIFIERS) {
            match TypeKind::try_from(token) {
                Ok(TypeKind::Int) => {
                    if self.advance_if(&[TokenKind::LeftBracket]).is_some() {
                        match advance!(self, &[TokenKind::RightBracket]) {
                            Ok(_) => (),
                            Err(e) => return Some(Err(e)),
                        };
                        Some(Ok(Type {
                            ty: TypeKind::IntArray,
                            token,
                        }))
                    } else {
                        Some(Ok(Type {
                            ty: TypeKind::Int,
                            token,
                        }))
                    }
                }
                Ok(TypeKind::Char) => {
                    if self.advance_if(&[TokenKind::LeftBracket]).is_some() {
                        match advance!(self, &[TokenKind::RightBracket]) {
                            Ok(_) => (),
                            Err(e) => return Some(Err(e)),
                        };
                        Some(Ok(Type {
                            ty: TypeKind::CharArray,
                            token,
                        }))
                    } else {
                        Some(Ok(Type {
                            ty: TypeKind::Int,
                            token,
                        }))
                    }
                }
                Ok(ty) => Some(Ok(Type { ty, token })),
                Err(e) => Some(Err(e)),
            }
        } else {
            None
        }
    }
}

pub const TYPE_SPECIFIERS: [TokenKind; 6] = [
    TokenKind::Void,
    TokenKind::Boolean,
    TokenKind::Char,
    TokenKind::Int,
    TokenKind::String,
    TokenKind::Id,
];
