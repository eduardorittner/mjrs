use lexer::token::{Token, TokenKind, TokenResult};

use crate::ast::{Expr, Node, NodeErr, NodeKind, NodeResult};

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
    pub fn parse(self) -> Node {
        todo!()
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

    fn expr(&mut self) -> NodeResult {
        let mut expr = self.primary_expr()?;

        while let Ok(tok) = self.tokens[self.idx] {
            if tok.kind.is_binary_operator() {
                self.idx += 1;
                let rhs = self.expr()?;

                expr = Node {
                    kind: NodeKind::Expr(Expr::Binary {
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    }),
                    token: tok,
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parses primary expressions, which are either literals or of the form '(' <expr> ')'
    fn primary_expr(&'src mut self) -> NodeResult {
        let primary_kinds = vec![
            TokenKind::True,
            TokenKind::False,
            TokenKind::IntLiteral,
            TokenKind::CharLiteral,
            TokenKind::This,
        ];

        let token = self.advance(&primary_kinds)?;

        match token.kind {
            TokenKind::True => Ok(Node {
                token,
                kind: NodeKind::Expr(Expr::True),
            }),
            TokenKind::False => Ok(Node {
                token,
                kind: NodeKind::Expr(Expr::False),
            }),
            TokenKind::IntLiteral => Ok(Node {
                token,
                kind: NodeKind::Expr(Expr::IntLiteral),
            }),
            TokenKind::CharLiteral => Ok(Node {
                token,
                kind: NodeKind::Expr(Expr::CharLiteral),
            }),
            TokenKind::This => Ok(Node {
                token,
                kind: NodeKind::Expr(Expr::This),
            }),
            // TODO: parse '('
            kind => Err(NodeErr::Unexpected {
                expected: primary_kinds,
                actual: token,
            }),
        }
    }

    fn unary_expr(&mut self) -> NodeResult {
        todo!()
    }

    fn binary_expr(&mut self) -> NodeResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use lexer::token::{Token, TokenError, TokenKind, TokenResult};

    use crate::{
        Parser,
        ast::{Expr, Node, NodeKind, NodeResult},
    };

    fn test_parse_case(
        input: &str,
        tokens: Vec<TokenResult>,
        parse_fn: &mut Box<dyn FnMut(&mut Parser) -> NodeResult>,
        expected: NodeResult,
    ) {
        let mut parser = Parser::new(input, &tokens);
        let result = parse_fn(&mut parser);

        pretty_assertions::assert_eq!(expected, result);
    }

    #[test]
    fn primary_expr() {
        test_parse_case(
            "true",
            vec![Ok(Token {
                kind: TokenKind::True,
                range: (0, 4),
                line: 1,
            })],
            &mut Box::new(Parser::primary_expr as (&dyn FnMut(&mut Parser) -> NodeResult)),
            Ok(Node {
                kind: NodeKind::Expr(Expr::True),
                token: Token {
                    kind: TokenKind::True,
                    range: (0, 4),
                    line: 1,
                },
            }),
        );
    }
}
