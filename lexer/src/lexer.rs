use std::process::exit;

use crate::token::{
    Token,
    TokenKind::{self, *},
};

pub struct Lexer<'src> {
    input: &'src str,
    /// Current position inside `input`
    idx: usize,
}

impl<'src> Lexer<'src> {
    pub fn lex(input: &'src str) -> Vec<Token> {
        let lexer = Lexer::new(input);
        lexer.collect()
    }

    pub fn new(input: &'src str) -> Lexer {
        Lexer { input, idx: 0 }
    }

    /// Returns a subslice of `input` starting at `idx`
    fn rest(&self) -> &str {
        &self.input[self.idx..]
    }

    /// Returns a new token and updates `self.idx`
    fn make_token(&mut self, kind: TokenKind, len: usize) -> Token {
        println!("making token {kind:?}");
        let token = Token {
            kind,
            range: (self.idx, self.idx + len),
        };
        self.idx += len;
        token
    }

    /// Lexes tokens which may have 1 or 2 characters
    fn lex_two_char_ops(&mut self, c1: char) -> Option<Token> {
        let mut rest = self.rest().chars();
        // Skip over `c1`
        rest.next();
        println!("{}", self.rest());
        if let Some(c2) = rest.next() {
            match (c1, c2) {
                ('=', '=') => Some(self.make_token(EqEq, 2)),
                ('=', _) => Some(self.make_token(Eq, 1)),
                ('!', '=') => Some(self.make_token(NotEq, 2)),
                ('!', _) => Some(self.make_token(Not, 1)),
                ('<', '=') => Some(self.make_token(LessEq, 2)),
                ('<', _) => Some(self.make_token(Less, 1)),
                ('>', '=') => Some(self.make_token(GreaterEq, 2)),
                ('>', _) => Some(self.make_token(Greater, 1)),
                ('&', '&') => Some(self.make_token(And, 2)),
                ('|', '|') => Some(self.make_token(Or, 2)),
                _ => panic!("Not valid token: '{c1}{c2}'"),
            }
        } else {
            match c1 {
                '=' => Some(self.make_token(Eq, 1)),
                '!' => Some(self.make_token(Not, 1)),
                '<' => Some(self.make_token(Less, 1)),
                '>' => Some(self.make_token(Greater, 1)),
                _ => unreachable!(),
            }
        }
    }

    // Lexes tokens which start with an alphabetic char
    fn lex_keyword_or_id(&mut self) -> Option<Token> {
        let mut rest = self.rest().chars().peekable();

        assert!(rest.peek().is_some_and(|c| c.is_ascii_alphabetic()));

        let len = if let Some(len) =
            rest.position(|c| !(c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_'))
        {
            len
        } else {
            self.rest().len()
        };

        Some(match &self.rest()[..len] {
            "if" => self.make_token(If, len),
            "else" => self.make_token(Else, len),
            "while" => self.make_token(While, len),
            "class" => self.make_token(Class, len),
            "new" => self.make_token(New, len),
            "return" => self.make_token(Return, len),
            "public" => self.make_token(Public, len),
            "private" => self.make_token(Private, len),
            "this" => self.make_token(This, len),
            "int" => self.make_token(Int, len),
            "boolean" => self.make_token(Boolean, len),
            "void" => self.make_token(Void, len),
            "false" => self.make_token(False, len),
            "true" => self.make_token(True, len),
            _ => self.make_token(Id, len),
        })
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        // Note: since `self.rest()` borrows from `self.idx`, we can't update `self.idx` while a
        // borrow to `self.rest()` is still active, so we end up having to call `self.rest()` twice
        let rest = self.rest();
        if let Some(skipped) = rest.find(|c: char| !c.is_whitespace()) {
            self.idx += skipped;
        }

        let mut rest = self.rest().chars().peekable();

        if let Some(c) = rest.next() {
            match c {
                // Unambiguous single-char tokens
                '+' => Some(self.make_token(Plus, 1)),
                '-' => Some(self.make_token(Minus, 1)),
                '*' => Some(self.make_token(Star, 1)),
                // TODO: handle comments
                '/' => Some(self.make_token(Slash, 1)),
                '(' => Some(self.make_token(LeftParen, 1)),
                ')' => Some(self.make_token(RightParen, 1)),
                '{' => Some(self.make_token(LeftBrace, 1)),
                '}' => Some(self.make_token(RightBrace, 1)),
                '[' => Some(self.make_token(LeftBracket, 1)),
                ']' => Some(self.make_token(RightBracket, 1)),
                ',' => Some(self.make_token(Comma, 1)),
                ';' => Some(self.make_token(Semicolon, 1)),
                // Single or double char tokens
                '=' | '!' | '<' | '>' | '&' | '|' => self.lex_two_char_ops(c),
                // Identifiers or keywords
                c if c.is_ascii_alphabetic() => self.lex_keyword_or_id(),
                // TODO: make this return an error?
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenKind::*;

    #[test]
    fn single_char_tokens() {
        let source = "+-*/(){}[],;";
        let tokens = Lexer::lex(source);
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn double_char_tokens() {
        let source = "== != <= >= && || = ! < >";
        let tokens = Lexer::lex(source);
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn keywords() {
        let source =
            "if else while class new return public private this int boolean void false true";
        let tokens = Lexer::lex(source);
        insta::assert_debug_snapshot!(tokens);
    }
}
