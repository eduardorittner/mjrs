use lexer::token::{Token, TokenKind, TokenResult};

use crate::ast::{
    Expr, MainMethodDecl, MethodDecl, Node, NodeErr, NodeKind, NodeResult, Print, RegularMethodDecl,
};

pub mod ast;

pub struct Parser<'src> {
    /// Needed for reading the values from literals and identifiers
    input: &'src str,

    tokens: &'src Vec<TokenResult>,

    /// Current position inside `tokens`
    idx: usize,
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
        let class_decl = match *class_node.kind {
            NodeKind::ClassDecl(decl) => decl,
            _ => panic!("Expected ClassDecl"),
        };
        Ok(Node {
            kind: Box::new(NodeKind::Program(crate::ast::Program {
                classes: vec![class_decl],
            })),
            token: class_node.token,
        })
    }

    fn class_decl(&mut self) -> NodeResult {
        // Parse "class" keyword
        let class_token = self.advance(&[TokenKind::Class])?;

        // Parse class name (identifier)
        let name_token = self.advance(&[TokenKind::Id])?;
        let name = crate::ast::Id {
            name: self.get_token_value(name_token).to_string(),
            token: name_token,
        };

        // Parse "{"
        let compound_start = self.advance(&[TokenKind::LeftBrace])?;

        // Parse class body
        let mut var_decls = Vec::new();
        let mut method_decls = Vec::new();

        // Parse variable and method declarations
        while let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::Int | TokenKind::Char | TokenKind::Boolean => {
                    // Check if it's a variable or method declaration
                    // For now, let's just parse variable declarations
                    let var_decl_node = self.var_decl()?;
                    let var_decl = match *var_decl_node.kind {
                        NodeKind::VarDecl(decl) => decl,
                        _ => panic!("Expected VarDecl"),
                    };
                    var_decls.push(var_decl);
                }
                TokenKind::Public => {
                    // Method declaration
                    let method_decl_node = self.method_decl()?;
                    match *method_decl_node.kind {
                        NodeKind::MethodDecl(decl) => method_decls.push(decl),
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
        self.advance(&[TokenKind::RightBrace])?;

        Ok(Node {
            kind: Box::new(NodeKind::ClassDecl(crate::ast::ClassDecl {
                name: Box::new(name),
                token: class_token,
                var_decls,
                method_decls,
                body: crate::ast::Compound {
                    stmts: vec![],
                    token: compound_start,
                },
            })),
            token: class_token,
        })
    }

    fn var_decl(&mut self) -> NodeResult {
        // Parse type
        println!("{:?}", self.tokens[self.idx]);
        let type_token = self.advance(&[TokenKind::Int, TokenKind::Char, TokenKind::Boolean])?;
        let ty = match type_token.kind {
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
            _ => unreachable!(),
        };

        // Parse first identifier
        let name_token = self.advance(&[TokenKind::Id])?;
        let name = crate::ast::Id {
            name: self.get_token_value(name_token).to_string(),
            token: name_token,
        };

        // Parse optional initializer
        let mut init = None;
        if let Some(Ok(eq_token)) = self.peek() {
            if eq_token.kind == TokenKind::Eq {
                self.idx += 1; // consume "="
                init = Some(Box::new(self.expr()?));
            }
        }

        // Check for comma-separated declarations
        if let Some(Ok(comma_token)) = self.peek() {
            if comma_token.kind == TokenKind::Comma {
                // For now, we'll just parse the first declaration and ignore the rest
                while let Some(Ok(token)) = self.peek() {
                    if token.kind == TokenKind::Comma {
                        self.idx += 1; // consume comma
                        self.advance(&[TokenKind::Id])?; // consume identifier

                        // Check for optional initializer
                        if let Some(Ok(eq_token)) = self.peek() {
                            if eq_token.kind == TokenKind::Eq {
                                self.idx += 1; // consume "="
                                self.expr()?; // consume expression
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        // Parse ";"
        self.advance(&[TokenKind::Semicolon])?;

        Ok(Node {
            kind: Box::new(NodeKind::VarDecl(crate::ast::VarDecl {
                ty: Box::new(ty),
                name: Box::new(name),
                init,
            })),
            token: type_token,
        })
    }

    fn get_token_value(&self, token: Token) -> &str {
        &self.input[token.range.0..token.range.1]
    }

    /// Consumes the next token, returning an error if the token kind is not present in `expected`
    fn advance(&mut self, expected: &[TokenKind]) -> Result<Token, NodeErr> {
        match self.tokens[self.idx] {
            Ok(tok) => {
                if expected.contains(&tok.kind) {
                    self.idx += 1;
                    Ok(tok)
                } else {
                    // TODO: should we advance `self.idx` here?
                    Err(NodeErr::Unexpected {
                        expected: Vec::from(expected),
                        actual: tok,
                    })
                }
            }
            Err(e) => Err(NodeErr::LexErr(e)),
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
    fn peek_n(&mut self, n: usize) -> Option<TokenResult> {
        let idx = self.idx + n - 1;
        if idx != self.tokens.len() {
            Some(self.tokens[idx])
        } else {
            None
        }
    }

    fn expr(&mut self) -> NodeResult {
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

    fn parse_expr(&mut self, min_precedence: u8) -> NodeResult {
        let mut left = self.primary_expr()?;

        while let Some(Ok(token)) = self.peek() {
            if !token.kind.is_binary_operator() {
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

            // Save the token before moving left
            let left_token = left.token;

            left = Node {
                kind: Box::new(NodeKind::Expr(Expr::Binary {
                    op: token.kind,
                    left: Box::new(left),
                    right: Box::new(right),
                })),
                token: left_token,
            };
        }

        Ok(left)
    }

    /// Parses primary expressions, which are either literals or of the form '(' <expr> ')'
    fn primary_expr(&mut self) -> NodeResult {
        let mut expr = self.primary_expr_without_field_access()?;

        // Handle field access expressions
        while let Some(Ok(dot_token)) = self.peek() {
            if dot_token.kind == TokenKind::Dot {
                self.idx += 1; // consume '.'

                // Expect an identifier after the dot
                let field_token = self.advance(&[TokenKind::Id])?;
                let field = crate::ast::Id {
                    name: self.get_token_value(field_token).to_string(),
                    token: field_token,
                };

                expr = Node {
                    token: expr.token,
                    kind: Box::new(NodeKind::Expr(Expr::FieldAccess {
                        object: Box::new(expr),
                        field: Box::new(field),
                    })),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parses primary expressions without field access
    fn primary_expr_without_field_access(&mut self) -> NodeResult {
        // Handle unary operators
        if let Some(Ok(token)) = self.peek() {
            if token.kind.is_unary_operator() {
                self.idx += 1; // consume operator
                let operand = self.primary_expr()?;
                return Ok(Node {
                    token,
                    kind: Box::new(NodeKind::Expr(Expr::Unary {
                        op: token.kind,
                        operand: Box::new(operand),
                    })),
                });
            }
        }

        if let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::True => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::True)),
                    })
                }
                TokenKind::False => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::False)),
                    })
                }
                TokenKind::IntLiteral => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::IntLiteral)),
                    })
                }
                TokenKind::CharLiteral => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::CharLiteral)),
                    })
                }
                TokenKind::This => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::This)),
                    })
                }
                TokenKind::Id => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::Identifier(Box::new(
                            crate::ast::Id {
                                name: self.get_token_value(token).to_string(),
                                token,
                            },
                        )))),
                    })
                }
                TokenKind::StringLiteral => {
                    self.idx += 1;
                    Ok(Node {
                        token,
                        kind: Box::new(NodeKind::Expr(Expr::True)), // Placeholder for now
                    })
                }
                TokenKind::LeftParen => {
                    self.idx += 1; // consume '('
                    let expr = self.expr()?;
                    self.advance(&[TokenKind::RightParen])?; // consume ')'
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
                }),
            }
        } else {
            todo!("emit Unexepcted error here")
        }
    }

    fn unary_expr(&mut self) -> NodeResult {
        todo!()
    }

    fn binary_expr(&mut self) -> NodeResult {
        todo!()
    }

    fn method_decl(&mut self) -> NodeResult {
        // Parse "public"
        let public_token = self.advance(&[TokenKind::Public])?;

        // Parse "static"
        self.advance(&[TokenKind::Static])?;

        // Parse return type
        let return_type_token = self.advance(&[
            TokenKind::Void,
            TokenKind::Int,
            TokenKind::Char,
            TokenKind::Boolean,
        ])?;
        let return_type = match return_type_token.kind {
            TokenKind::Void => crate::ast::Type {
                ty: crate::ast::TypeKind::Custom("void".to_string()),
                token: return_type_token,
            },
            TokenKind::Int => crate::ast::Type {
                ty: crate::ast::TypeKind::Int,
                token: return_type_token,
            },
            TokenKind::Char => crate::ast::Type {
                ty: crate::ast::TypeKind::Char,
                token: return_type_token,
            },
            TokenKind::Boolean => crate::ast::Type {
                ty: crate::ast::TypeKind::Boolean,
                token: return_type_token,
            },
            _ => unreachable!(),
        };

        // Parse method name
        let name_token = self.advance(&[TokenKind::Id, TokenKind::Main])?;
        let name = crate::ast::Id {
            name: self.get_token_value(name_token).to_string(),
            token: name_token,
        };

        // Parse "("
        self.advance(&[TokenKind::LeftParen])?;

        // Parse parameter list
        let param_list = self.param_list();

        // Parse ")"
        self.advance(&[TokenKind::RightParen])?;

        // Parse method body
        let body = self.compound_stmt();

        println!("{name_token:?}");

        Ok(Node {
            kind: Box::new({
                if name_token.kind == TokenKind::Main {
                    NodeKind::MethodDecl(MethodDecl::Main(MainMethodDecl {
                        ty: Box::new(return_type),
                        name: Box::new(name),
                        param_list: Box::new(param_list),
                        body,
                        token: public_token,
                    }))
                } else {
                    NodeKind::MethodDecl(MethodDecl::Regular(RegularMethodDecl {
                        ty: Box::new(return_type),
                        name: Box::new(name),
                        param_list: Box::new(param_list),
                        body,
                        token: public_token,
                    }))
                }
            }),
            token: public_token,
        })
    }

    fn param_list(&mut self) -> crate::ast::ParamList {
        // Parse parameter list: (Type Id, Type Id, ...)
        let mut params = Vec::new();

        // Check if there are any parameters
        if let Some(Ok(token)) = self.peek() {
            if token.kind != TokenKind::RightParen {
                // Parse first parameter
                let type_token = self
                    .advance(&[
                        TokenKind::Int,
                        TokenKind::Char,
                        TokenKind::Boolean,
                        TokenKind::String,
                    ])
                    .expect("Expected parameter type");

                // Check for array type
                let mut param_type = match type_token.kind {
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
                        self.advance(&[TokenKind::RightBracket])
                            .expect("Expected ']'");
                        // For now, we'll just keep the base type and ignore the array part
                    }
                }

                let name_token = self
                    .advance(&[TokenKind::Id])
                    .expect("Expected parameter name");
                let param_name = crate::ast::Id {
                    name: self.get_token_value(name_token).to_string(),
                    token: name_token,
                };

                params.push((param_type, param_name));

                // Parse additional parameters
                while let Some(Ok(comma_token)) = self.peek() {
                    if comma_token.kind == TokenKind::Comma {
                        self.idx += 1; // consume comma

                        let type_token = self
                            .advance(&[
                                TokenKind::Int,
                                TokenKind::Char,
                                TokenKind::Boolean,
                                TokenKind::String,
                            ])
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
                                self.advance(&[TokenKind::RightBracket])
                                    .expect("Expected ']'");
                                // For now, we'll just keep the base type and ignore the array part
                            }
                        }

                        let name_token = self
                            .advance(&[TokenKind::Id])
                            .expect("Expected parameter name");
                        let param_name = crate::ast::Id {
                            name: self.get_token_value(name_token).to_string(),
                            token: name_token,
                        };

                        params.push((param_type, param_name));
                    } else {
                        break;
                    }
                }
            }
        }

        crate::ast::ParamList { params }
    }

    fn compound_stmt(&mut self) -> crate::ast::Compound {
        // Parse "{"
        let compound_start = self.advance(&[TokenKind::LeftBrace]).expect("Expected '{'");

        let mut stmts = Vec::new();

        // Parse statements until "}"
        while let Some(Ok(token)) = self.peek() {
            match token.kind {
                TokenKind::Int | TokenKind::Char | TokenKind::Boolean => {
                    // Variable declaration
                    let var_decl_node = self.var_decl().expect("Expected variable declaration");
                    let var_decl = match *var_decl_node.kind {
                        NodeKind::VarDecl(decl) => decl,
                        _ => panic!("Expected VarDecl"),
                    };
                    stmts.push(crate::ast::Statement::VarDecl(var_decl));
                }
                TokenKind::Print => {
                    // Print statement
                    let print = self
                        .advance(&[TokenKind::Print])
                        .expect("checked before-hand");
                    self.advance(&[TokenKind::LeftParen]).expect("Expected '('");
                    let expr = self.expr().expect("Expected expression");
                    self.advance(&[TokenKind::RightParen])
                        .expect("Expected ')'");
                    self.advance(&[TokenKind::Semicolon]).expect("Expected ';'");

                    stmts.push(crate::ast::Statement::Print(Print {
                        item: Box::new(expr),
                        token: print,
                    }));
                }
                TokenKind::Id => {
                    // Either expression statement or declaration
                    // TODO: fuck you
                    // Expression statement (for now, just consume it)

                    match self.peek_n(1) {
                        Some(Ok(t)) => match t.kind {
                            TokenKind::Id => todo!("var decl"),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    }
                }
                TokenKind::RightBrace => {
                    break;
                }
                _ => {
                    // For now, just break on unknown tokens
                    break;
                }
            }
        }

        // Parse "}"
        self.advance(&[TokenKind::RightBrace])
            .expect("Expected '}'");

        crate::ast::Compound {
            stmts,
            token: compound_start,
        }
    }
}

#[cfg(test)]
mod tests {
    use lexer::token::{Coords, Token, TokenError, TokenKind, TokenResult};

    use crate::{
        Parser,
        ast::{Expr, Node, NodeKind, NodeResult},
    };

    struct ParseCaseArgs<'src> {
        input: &'src str,
        tokens: &'src Vec<TokenResult>,
        expected: NodeResult,
    }

    // Note: we're using a macro here instead of a function which takes a generic function
    // fn(&mut self) -> NodeResult since for some reason the lifetimes weren't matching.
    // With macros we don't have this problem since we pass the name of the function, not the
    // function itself.
    macro_rules! test_parse_case {
        ($fn_name:ident, $args:ident) => {
            let mut parser = Parser::new($args.input, $args.tokens);
            let result = Parser::$fn_name(&mut parser);

            pretty_assertions::assert_eq!($args.expected, result);
        };
    }

    #[test]
    fn primary_expr() {
        let args = ParseCaseArgs {
            input: "true",
            tokens: &vec![Ok(Token {
                kind: TokenKind::True,
                range: (0, 4),
                coords: Coords::new(1, 0),
            })],
            expected: Ok(Node {
                kind: Box::new(NodeKind::Expr(Expr::True)),
                token: Token {
                    kind: TokenKind::True,
                    range: (0, 4),
                    coords: Coords::new(1, 0),
                },
            }),
        };
        test_parse_case!(expr, args);
    }
}
