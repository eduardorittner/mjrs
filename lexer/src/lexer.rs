use std::process::exit;

use crate::token::{
    Token, TokenError,
    TokenKind::{self, *},
    TokenResult,
};

pub struct Lexer<'src> {
    input: &'src str,
    /// Current position inside `input`
    idx: usize,
    /// Current line number inside `input`
    line: usize,
    /// The byte index of the start of `line`
    line_offset: usize,
}

impl<'src> Lexer<'src> {
    pub fn lex(input: &'src str) -> Vec<TokenResult> {
        let lexer = Lexer::new(input);
        lexer.collect()
    }

    pub fn new(input: &'src str) -> Lexer {
        Lexer {
            input,
            idx: 0,
            line: 1, // SLY considers lines as 1-indexed
            line_offset: 0,
        }
    }

    /// Returns a subslice of `input` starting at `idx`
    fn rest(&self) -> &str {
        &self.input[self.idx..]
    }

    /// Returns a new token and updates `self.idx`
    fn make_token(&mut self, kind: TokenKind, len: usize) -> Option<TokenResult> {
        let token = Token {
            kind,
            range: (self.idx, self.idx + len),
            line: self.line,
        };
        self.idx += len;
        Some(Ok(token))
    }

    fn make_err(&mut self) -> Option<TokenResult> {
        println!("idx {}, line offset {}", self.idx, self.line_offset);
        let token = TokenError {
            c: self
                .rest()
                .chars()
                .next()
                .expect("Token error should always happen on a char"),
            offset: self.idx - self.line_offset,
            line: self.line,
        };

        self.idx += 1;
        Some(Err(token))
    }

    /// Lexes tokens which may have 1 or 2 characters
    fn lex_two_char_ops(&mut self, c1: char) -> Option<TokenResult> {
        let mut rest = self.rest().chars();
        // Skip over `c1`
        rest.next();
        if let Some(c2) = rest.next() {
            match (c1, c2) {
                ('=', '=') => self.make_token(EqEq, 2),
                ('=', _) => self.make_token(Eq, 1),
                ('!', '=') => self.make_token(NotEq, 2),
                ('!', _) => self.make_token(Not, 1),
                ('<', '=') => self.make_token(LessEq, 2),
                ('<', _) => self.make_token(Less, 1),
                ('>', '=') => self.make_token(GreaterEq, 2),
                ('>', _) => self.make_token(Greater, 1),
                ('&', '&') => self.make_token(And, 2),
                ('|', '|') => self.make_token(Or, 2),
                _ => panic!("Not valid token: '{c1}{c2}'"),
            }
        } else {
            match c1 {
                '=' => self.make_token(Eq, 1),
                '!' => self.make_token(Not, 1),
                '<' => self.make_token(Less, 1),
                '>' => self.make_token(Greater, 1),
                _ => unreachable!(),
            }
        }
    }

    // Lexes tokens which start with an alphabetic char
    fn lex_keyword_or_id(&mut self) -> Option<TokenResult> {
        let mut rest = self.rest().chars().peekable();

        assert!(rest.peek().is_some_and(|c| c.is_ascii_alphabetic()));

        let len = if let Some(len) =
            rest.position(|c| !(c.is_ascii_digit() || c.is_ascii_alphabetic() || c == '_'))
        {
            len
        } else {
            self.rest().len()
        };

        match &self.rest()[..len] {
            "if" => self.make_token(If, len),
            "else" => self.make_token(Else, len),
            "while" => self.make_token(While, len),
            "for" => self.make_token(For, len),
            "main" => self.make_token(Main, len),
            "class" => self.make_token(Class, len),
            "new" => self.make_token(New, len),
            "return" => self.make_token(Return, len),
            "public" => self.make_token(Public, len),
            "private" => self.make_token(Private, len),
            "static" => self.make_token(Static, len),
            "this" => self.make_token(This, len),
            "print" => self.make_token(Print, len),
            "assert" => self.make_token(Assert, len),
            "String" => self.make_token(String, len),
            "int" => self.make_token(Int, len),
            "boolean" => self.make_token(Boolean, len),
            "void" => self.make_token(Void, len),
            "false" => self.make_token(False, len),
            "true" => self.make_token(True, len),
            _ => self.make_token(Id, len),
        }
    }

    fn lex_int_literal(&mut self) -> Option<TokenResult> {
        let mut rest = self.rest().chars().peekable();

        assert!(rest.peek().is_some_and(|c| c.is_ascii_digit()));

        let len = if let Some(len) = rest.position(|c| !c.is_ascii_digit()) {
            len
        } else {
            self.rest().len()
        };

        self.make_token(IntLiteral, len)
    }

    fn lex_string(&mut self) -> Option<TokenResult> {
        let mut rest = self.rest().chars();
        assert!(rest.next().is_some_and(|c| c == '"'));

        let len = if let Some(len) = rest.position(|c| c == '"') {
            len
        } else {
            self.rest().len()
        };

        self.make_token(StringLiteral, len + 2)
    }

    fn skip_whitespace(&mut self) {
        let rest = self.rest();
        if let Some(skipped) = rest.find(|c: char| !c.is_whitespace()) {
            // Count the number of newlines inside the skipped whitespace
            let newlines = rest
                .chars()
                .enumerate()
                .take(skipped)
                .filter(|(_idx, c)| *c == '\n');

            let last_offset = newlines.clone().last().map(|(idx, _c)| idx);
            let count = newlines.count();

            self.line += count;
            self.line_offset = match last_offset {
                Some(offset) => offset + self.idx,
                None => self.line_offset,
            };
            self.idx += skipped;
        }
    }

    fn skip_comments(&mut self) {
        let rest = self.rest();
        if rest.starts_with("//") {
            if let Some(len) = rest.find('\n') {
                self.idx += len;
                self.line += 1;
                self.line_offset = self.idx;
            } else {
                self.idx = self.input.len();
            }
        } else if rest.starts_with("/*") {
            if let Some(len) = rest.find("*/") {
                self.idx += len + 1;
                // TODO: update line info here
            } else {
                self.idx = self.input.len();
            }
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    // TODO: create alias for Result<Token, TokenError>
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.input.len() {
            return None;
        }

        // Skip whitespace
        // Note: since `self.rest()` borrows from `self.idx`, we can't update `self.idx` while a
        // borrow to `self.rest()` is still active, so we end up having to call `self.rest()` twice
        self.skip_whitespace();
        self.skip_comments();

        let mut rest = self.rest().chars().peekable();

        if let Some(c) = rest.next() {
            match c {
                // Unambiguous single-char tokens
                '+' => self.make_token(Plus, 1),
                '-' => self.make_token(Minus, 1),
                '*' => self.make_token(Star, 1),
                '%' => self.make_token(Mod, 1),
                // TODO: handle comments
                '/' => self.make_token(Slash, 1),
                '(' => self.make_token(LeftParen, 1),
                ')' => self.make_token(RightParen, 1),
                '{' => self.make_token(LeftBrace, 1),
                '}' => self.make_token(RightBrace, 1),
                '[' => self.make_token(LeftBracket, 1),
                ']' => self.make_token(RightBracket, 1),
                ',' => self.make_token(Comma, 1),
                ';' => self.make_token(Semicolon, 1),
                '.' => self.make_token(Dot, 1),
                // Single or double char tokens
                '=' | '!' | '<' | '>' | '&' | '|' => self.lex_two_char_ops(c),
                // Identifiers or keywords
                c if c.is_ascii_alphabetic() => self.lex_keyword_or_id(),
                // Int literals
                c if c.is_ascii_digit() => self.lex_int_literal(),
                '"' => self.lex_string(),
                // TODO: make this return an error?
                _ => self.make_err(),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

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
