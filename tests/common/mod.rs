#![allow(dead_code)]

use std::io::Cursor;

use mini_lisp::{
    evaluator::Evaluator,
    parser::Parser,
    scanner::{Scanner, token::Token},
};

pub fn setup_parser(input: &str) -> Parser {
    let mut cursor = Cursor::new(input);
    let mut scanner = Scanner::new();
    let mut tokens: Vec<Token> = vec![];
    loop {
        match scanner.scan_line(&mut cursor) {
            Ok(line_tokens) => match line_tokens {
                Some(line_tokens) => tokens.extend(line_tokens),
                None => break,
            },
            Err(e) => panic!("Scanner error: {}", e.to_string()),
        }
    }
    Parser::new(tokens)
}

pub fn setup_evaluator(input: &str) -> Evaluator {
    let mut parser = setup_parser(input);
    let program = parser.parse().unwrap();
    Evaluator::new(program)
}

pub fn assert_evaluation(input: &str, expected_output: &[&str]) {
    let mut buffer = Cursor::new(Vec::new());
    let mut evaluator = setup_evaluator(input);
    assert!(evaluator.evaluate(&mut buffer).is_ok());
    let output = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(output, expected_output.join("\n") + "\n");
}
