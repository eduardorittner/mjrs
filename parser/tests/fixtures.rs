use rstest::rstest;
use std::{fs, path::PathBuf};

#[rstest]
fn fixtures(#[files("../fixtures/parser/*.in")] path: PathBuf) {
    use lexer::lexer::Lexer;
    use parser::{Parser, ast::Show};

    let mut output_path = path.clone();
    output_path.set_extension("out");
    let input = fs::read_to_string(path).unwrap();
    let output = fs::read_to_string(output_path).unwrap();

    let tokens = &Lexer::lex(&input);
    let mut parser = Parser::new(&input, tokens);
    let ast = parser.parse().expect("Failed to parse");

    pretty_assertions::assert_str_eq!(output, ast.show(&input, 0));
}
