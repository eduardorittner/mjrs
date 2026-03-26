use std::{fs, process::exit};

use gumdrop::Options;
use lexer::{lexer::Lexer, token::fmt_tokens};
use parser::{Parser, ast::Show};

#[derive(Debug, Options)]
struct Opts {
    #[options(free)]
    file: String,

    // Mutually-exclusive options
    lex: bool,
    parse: bool,
}

impl Opts {
    fn is_valid(&self) -> Result<(), String> {
        if self.lex && self.parse {
            return Err("--lex and --parse are mutually exclusive options".to_string());
        }
        Ok(())
    }
}

fn main() {
    let opts = Opts::parse_args_default_or_exit();

    if let Err(e) = opts.is_valid() {
        println!("{e}");
        exit(1)
    }

    let file_contents =
        fs::read_to_string(&opts.file).expect(&format!("Couldn't read file {}", &opts.file));

    if opts.lex {
        let tokens = Lexer::lex(&file_contents);
        println!("{}", fmt_tokens(&tokens, &file_contents));
    } else if opts.parse {
        let tokens = Lexer::lex(&file_contents);
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse();

        match ast {
            Ok(ast) => println!("{}", ast.show(&file_contents, 0)),
            Err(e) => eprintln!("{e:?}"),
        }
    }
}
