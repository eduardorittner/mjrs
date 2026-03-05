use lexer::{lexer::Lexer, token};
use rstest::rstest;
use std::{fs, path::PathBuf};

#[rstest]
fn fixtures(#[files("../fixtures/in-out/*.in")] path: PathBuf) {
    let mut output_path = path.clone();
    output_path.set_extension("out");
    let input = fs::read_to_string(path).unwrap();
    let output = fs::read_to_string(output_path).unwrap();

    let result = token::fmt_tokens(&Lexer::lex(&input), &input);

    // TODO: maybe pretty diff here?
    pretty_assertions::assert_str_eq!(output, result);
}
