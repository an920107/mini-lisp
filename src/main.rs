use mini_lisp::{
    evaluator::Evaluator,
    parser::Parser,
    scanner::{Scanner, token::Token},
};

fn main() {
    let mut tokens: Vec<Token> = vec![];
    let mut scanner = Scanner::new();

    loop {
        match scanner.scan_line() {
            Ok(line_tokens) => match line_tokens {
                Some(line_tokens) => tokens.extend(line_tokens),
                None => break,
            },
            Err(e) => {
                eprintln!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("Tokens: {:#?}", tokens);
    }

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    if cfg!(debug_assertions) {
        println!("AST: {:#?}", program);
    }

    let mut evaluator = Evaluator::new(program);
    match evaluator.evaluate() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    }
}
