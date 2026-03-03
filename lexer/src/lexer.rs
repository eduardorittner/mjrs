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
        println!("{}", self.rest());
        if let Some(c2) = rest.next() {
            match (c1, c2) {
                ('=', '=') => Some(self.make_token(EqEq, 2)),
                ('=', _) => Some(self.make_token(Eq, 1)),
                ('!', '=') => Some(self.make_token(NotEq, 2)),
                ('!', _) => Some(self.make_token(Eq, 1)),
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
                '!' => Some(self.make_token(NotEq, 1)),
                '<' => Some(self.make_token(Less, 1)),
                '>' => Some(self.make_token(Greater, 1)),
                _ => unreachable!(),
            }
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rest = self.rest().chars().peekable();

        // Skip whitespace
        while rest.peek().is_some_and(|c| c.is_whitespace()) {
            rest.next();
            self.idx += 1;
        }

        if let Some(c) = rest.next() {
            match c {
                '+' => Some(self.make_token(Plus, 1)),
                '-' => Some(self.make_token(Minus, 1)),
                '*' => Some(self.make_token(Star, 1)),
                '/' => Some(self.make_token(Slash, 1)),
                '(' => Some(self.make_token(LeftParen, 1)),
                ')' => Some(self.make_token(RightParen, 1)),
                '{' => Some(self.make_token(LeftBrace, 1)),
                '}' => Some(self.make_token(RightBrace, 1)),
                '[' => Some(self.make_token(LeftBracket, 1)),
                ']' => Some(self.make_token(RightBracket, 1)),
                ',' => Some(self.make_token(Comma, 1)),
                ';' => Some(self.make_token(Semicolon, 1)),
                '=' | '!' | '<' | '>' | '&' | '|' => self.lex_two_char_ops(c),
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
}
